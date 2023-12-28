# Example of Go client

1. `cargo install --git https://github.com/bytecodealliance/wit-bindgen wit-bindgen-cli` to get `wit-bingen-cli`.
2. Install `TinyGo`.
3. Write guest code in `my-component.go`.
4. Run `wit-bindgen tiny-go ../spec` from `go-client-app` folder.
5. Move generated files to `src/my-world` folder (TODO: figure out better way that doesn't require moving files).
6. `GOPATH=$(pwd) tinygo build -target=wasi -o main.wasm my-component.go` from `go-client-app` folder
7. `wasm-tools component embed --world my-world ../spec main.wasm -o main.embed.wasm`
8. `wasm-tools component new main.embed.wasm --adapt ../client-app/wasi_snapshot_preview1.reactor.wasm -o main.component.wasm`
9. `wasm-tools validate main.component.wasm --features component-model` (Optional)
10. `cd ../brotli-encoder` and `cargo run --package brotli-encoder --release -- ../go-client-app/main.component.wasm ../levo-server/my-component-wasm.br`
11. Re-start `levo-server`
