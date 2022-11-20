# links
https://wasmbyexample.dev/examples/wasi-hello-world/wasi-hello-world.go.en-us.html

https://tinygo.org/getting-started/install/
https://github.com/WebAssembly/binaryen
https://wasmtime.dev

https://www.docker.com/blog/docker-wasm-technical-preview/

#install
export PATH=~/tinygo/bin:~/tinygo/binaryen/bin:$PATH
sudo xattr -r -d com.apple.quarantine ~/tinygo

# command
# tinygo build -wasm-abi=generic -target=wasi -o main.wasm main.go
           
docker run --rm -v $(pwd):/src tinygo/tinygo:0.26.0 tinygo build -target=wasi -o /src/main.wasm /src/main.go
wasmtime --dir . main.wasm

docker run --rm -v $(pwd):/src tinygo/tinygo:0.26.0 tinygo build -target=wasi -o /src/restservice.wasm /src/restservice.go
wasmtime --dir . main.wasm


# go command
GOOS=js GOARCH=wasm go build -o main.wasm main.go


