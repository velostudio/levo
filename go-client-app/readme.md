# Example of Go client

`cargo install --git https://github.com/bytecodealliance/wit-bindgen wit-bindgen-cli` to get `wit-bingen-cli`.  
Install `Go`.  
Install `TinyGo`.  

1. Write guest code in `my-component.go`
2. `export GO111MODULE=on`
3. Run `wit-bindgen tiny-go ../spec --out-dir=my-world` from `go-client-app` folder
4. `tinygo build -target=wasi -o main.wasm my-component.go` from `go-client-app` folder
5. `wasm-tools component embed --world my-world ../spec main.wasm -o main.embed.wasm`
6. `wasm-tools component new main.embed.wasm --adapt ../client-app/wasi_snapshot_preview1.reactor.wasm -o main.component.wasm`
7. `wasm-tools validate main.component.wasm --features component-model` (Optional)
8. `cd ../brotli-encoder` and `cargo run --package brotli-encoder --release -- ../go-client-app/main.component.wasm ../levo-server/my-component-wasm.br`

Re-start `levo-server`

There is `go.sh` that automates part of this readme.
