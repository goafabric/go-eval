#!/bin/sh
# Runs the service as a wasm component natively in the local kind cluster: the node's containerd runs it with the
# runwasi wasmtime shim, the image only contains the .wasm. Reachable at http://localhost:30800.
# Usage: ./deploy-kind-wasm.sh   (NODE, REGISTRY, SHIM_VERSION can be overridden)
set -eu
cd "$(dirname "$0")"

NODE="${NODE:-kind-control-plane}"
REGISTRY="${REGISTRY:-localhost:5001}"
SHIM_VERSION="${SHIM_VERSION:-0.6.1}"
IMAGE="$REGISTRY/callee-service-wasm:$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)"
case "$(docker exec "$NODE" uname -m)" in
    aarch64) ARCH=aarch64; PLATFORM=linux/arm64 ;;
    x86_64) ARCH=x86_64; PLATFORM=linux/amd64 ;;
    *) echo "unsupported node architecture" >&2; exit 1 ;;
esac

# once per cluster: install the shim on the node and register it as containerd runtime "wasmtime"
if ! docker exec "$NODE" test -x /usr/local/bin/containerd-shim-wasmtime-v1; then
    tmp="$(mktemp -d)"
    curl -fsSL "https://github.com/containerd/runwasi/releases/download/containerd-shim-wasmtime%2Fv$SHIM_VERSION/containerd-shim-wasmtime-$ARCH-linux-musl.tar.gz" | tar -xz -C "$tmp"
    docker cp "$tmp/containerd-shim-wasmtime-v1" "$NODE:/usr/local/bin/containerd-shim-wasmtime-v1"
    rm -rf "$tmp"
fi
if ! docker exec "$NODE" grep -q 'runtimes.wasmtime\]' /etc/containerd/config.toml; then
    printf '\n[plugins."io.containerd.grpc.v1.cri".containerd.runtimes.wasmtime]\n  runtime_type = "io.containerd.wasmtime.v1"\n' \
        | docker exec -i "$NODE" sh -c 'cat >> /etc/containerd/config.toml && systemctl restart containerd'
    sleep 5
fi

# the lib is built as cdylib so the component exports only wasi:http/incoming-handler (the shim picks the first of
# wasi:cli/run and wasi:http/incoming-handler it finds). The image platform is set to the node's, so the node pulls it.
rustup target add wasm32-wasip2
cargo rustc --lib --release --locked --target wasm32-wasip2 --crate-type cdylib
mkdir -p dist
cp target/wasm32-wasip2/release/callee_service.wasm dist/callee-service.wasm
docker buildx build --platform "$PLATFORM" --provenance=false --sbom=false -f Dockerfile.wasm -t "$IMAGE" --push .

kubectl apply -f k8s/callee-service-wasm.yaml
kubectl rollout restart deployment/callee-service-wasm
kubectl rollout status deployment/callee-service-wasm
echo "try: curl 'http://localhost:30800/callees/sayMyName?name=Heisenberg'"
