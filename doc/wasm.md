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
cat ./main.go
tinygo build -wasm-abi=generic -target=wasi -o main.wasm main.go
wasmtime --dir . main.wasm

# command
GOOS=js GOARCH=wasm go build -o main.wasm main.go


# error

error: could not find wasm-opt, set the WASMOPT environment variable to override
