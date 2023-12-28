# Example of Go client

1. `cargo install --git https://github.com/bytecodealliance/wit-bindgen wit-bindgen-cli` to get `wit-bingen-cli`.
2. Install `TinyGo`.
3. Temporary rename `spec` folder to `wit` and `my-world` to `host` in `host.wit` file (TODO: should be easy to avoid this step in the future).
4. Write guest code in `my-component.go`.
5. Run `wit-bindgen tiny-go ../wit` from `go-client-app` folder.
6. Move generated files to `src/host` folder (TODO: figure out better way that doesn't require moving files).
7. `GOPATH=$(pwd) tinygo build -target=wasi -o main.wasm my-component.go` from `go-client-app` folder
8. `wasm-tools component embed --world host ../wit main.wasm -o main.embed.wasm`
9. `wasm-tools component new main.embed.wasm --adapt ../client-app/wasi_snapshot_preview1.reactor.wasm -o main.component.wasm`
10. `wasm-tools validate main.component.wasm --features component-model` (Optional)
11. `cd ../brotli-encoder` and `cargo run --package brotli-encoder --release -- ../go-client-app/main.component.wasm ../levo-server/my-component-wasm.br`
12. Re-start `levo-server`
13. Rename back `wit` folder to `spec` and `host` to `my-world` in `host.wit` file
