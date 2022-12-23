# install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-wasi

# uninstall
rustup self uninstall

# http server
https://dev.to/steadylearner/how-to-use-rust-warp-web-framework-2b4e