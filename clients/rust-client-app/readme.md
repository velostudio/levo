# wasm [generating wasm file from rust with wit]

This is example repo for generating wasm file with wit definition. wit file has exports and imports.

```sh
./build.sh
```

## Notes

- wasm-tools can be obtained by `cargo install wasm-tools`
- wasmtime runtime should be compatible with `wasi_snapshot_preview1.reactor.wasm` adapter (has been committed to this repo). To get this file go to release archives in https://github.com/bytecodealliance/wasmtime
