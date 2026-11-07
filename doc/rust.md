# install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-wasi

# uninstall
rustup self uninstall

# http server
https://dev.to/steadylearner/how-to-use-rust-warp-web-framework-2b4e

# build
cargo build --release

# loco
https://github.com/loco-rs/loco
cargo install loco && cargo install sea-orm-cli
/Users/andreas/.cargo/bin/loco new
cd myapp
cargo loco start
                     

# wasi restservice
cd rust/restservice-wasi

cargo build --release --target wasm32-wasip2
wasmtime serve -S cli --addr 127.0.0.1:8080 target/wasm32-wasip2/release/restservice.wasm
curl http://localhost:8080/hello/yo
