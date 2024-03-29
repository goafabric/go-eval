# links
https://wasmbyexample.dev/examples/wasi-hello-world/wasi-hello-world.go.en-us.html
https://wasmtime.dev
https://www.docker.com/blog/docker-wasm-technical-preview/

# zsh functions

function tinygo-wasi() { docker run --rm -v $(pwd):/src -w /src/ tinygo/tinygo:0.26.0 tinygo build -target=wasi -wasm-abi=generic $1 }
function tinygo-run() { docker run --rm -v $(pwd):/src -w /src/ tinygo/tinygo:0.26.0 tinygo run $1 }

# run
tinygo-wasi main.go
wasmtime main.go.wasm

# go command
GOOS=js GOARCH=wasm go build -o main.wasm main.go

# ssl off
export GOINSECURE=*  && export GONOPROXY=*  && export GIT_SSL_NO_VERIFY=true

# string pass
https://www.wasm.builders/k33g_org/an-essay-on-the-bi-directional-exchange-of-strings-between-the-wasm-module-with-tinygo-and-nodejs-with-wasi-support-3i9h

# docker
https://docs.docker.com/desktop/wasm/

docker run --rm --name=saymname --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm32 goafabric/saymname:1.0.0

docker run --rm -p 8080:8080 --name=wasm-example --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm32 michaelirwin244/wasm-example
curl http://localhost:8080