# links
https://wasmbyexample.dev/examples/wasi-hello-world/wasi-hello-world.go.en-us.html
https://wasmtime.dev
https://www.docker.com/blog/docker-wasm-technical-preview/

# zsh functions

function tinygo-wasm() { docker run --rm -v $(pwd):/src tinygo/tinygo:0.26.0 tinygo build -target=wasi -o /src/$1.wasm /src/$1 }
function tinygo-run() { docker run --rm -v $(pwd):/src tinygo/tinygo:0.26.0 tinygo run /src/$1 }

# run


# go command
GOOS=js GOARCH=wasm go build -o main.wasm main.go


