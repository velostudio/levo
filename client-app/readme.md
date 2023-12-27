# wasm [generating wasm file from rust with wit]

This is example repo for generating wasm file with wit definition. wit file has exports and imports.

```
cargo build --target wasm32-wasi --release
```

to create intermediate wasm file.

then

```
wasm-tools component new ../target/wasm32-wasi/release/client_app.wasm -o my-component.wasm --adapt ./wasi_snapshot_preview1.reactor.wasm
```

to check that everything is alright run

```
wasm-tools component wit my-component.wasm
```

(should print wit definition with correct types)

## Notes

- wasm-tools can be obtained by `cargo install wasm-tools`
- wasmtime runtime should be compatible with `wasi_snapshot_preview1.reactor.wasm` adapter (has been committed to this repo). To get this file go to release archives in https://github.com/bytecodealliance/wasmtime
- use build.sh (in root of project) to automate brotli wasm file generation
