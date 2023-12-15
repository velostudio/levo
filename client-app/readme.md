# wasm [generating wasm file from rust with wit]

This is example repo for genereting wasm file with wit definition. wit file has exports and imports. `print` func defined in wit file.
Then, `print` function is called in `lib.rs` with ""Hello, world!" payload.


```
cargo build --target wasm32-wasi --release
```

to create intermediate wasm file.

then

```
wasm-tools component new ./target/wasm32-wasi/release/wit_play.wasm -o my-component.wasm --adapt ./wasi_snapshot_preview1.reactor.wasm
```

to check that everything is alright run

```
wasm-tools component wit my-component.wasm
```
(should print wit definition with correct types)

## Notes

- wasm-tools can be obtained by `cargo install wasm-tools`
- wasmtime runtime should be compatible with `wasi_snapshot_preview1.reactor.wasm` adapter. To get this file go to release archives in https://github.com/bytecodealliance/wasmtime
