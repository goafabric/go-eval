#!/bin/sh
# Cross-compiles a static linux binary on the host, packs it into an image and pushes it.
# Usage: ./build-image.sh [aarch64|x86_64]   (default: the host architecture)
# NO_PUSH=1 builds only, IMAGE=<registry/name> overrides the repository (default goafabric/callee-service-rust).
set -eu
cd "$(dirname "$0")"

ARCH="${1:-$(uname -m)}"
case "$ARCH" in
    arm64|aarch64) ARCH=aarch64; PLATFORM_ARCH=arm64 ;;
    x86_64|amd64) ARCH=x86_64; PLATFORM_ARCH=amd64 ;;
    *) echo "unsupported architecture: $ARCH" >&2; exit 1 ;;
esac
TARGET="$ARCH-unknown-linux-musl"
IMAGE="${IMAGE:-goafabric/callee-service-rust}:$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)"

rustup target add "$TARGET"
cargo build --release --locked --target "$TARGET"
mkdir -p dist
cp "target/$TARGET/release/callee-service" dist/callee-service
container build --arch "$PLATFORM_ARCH" -t "$IMAGE" .
echo "built $IMAGE"
[ -n "${NO_PUSH:-}" ] || container image push "$IMAGE"
