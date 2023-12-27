_WIP_

<img src="https://raw.githubusercontent.com/velostudio/levo/main/levo.png" width="128" />

https://staffengineer.github.io/blog/levo.html

## web as it should be

To re-generate brotli encoded wasm file, use the following commands, or run `build.sh`.

```bash
# build the client app with wasm32-wasi target
cargo build --package client-app --target wasm32-wasi --release
# patch the client app to support WASI and component model
wasm-tools component new ./target/wasm32-wasi/release/client_app.wasm -o ./target/my-component.wasm --adapt ./client-app/wasi_snapshot_preview1.reactor.wasm
# compress the patched wasm file
cargo run --package brotli-encoder --release -- ./target/my-component.wasm ./levo-server/my-component-wasm.br
```

To start demo server:

```bash
cd levo-server
cargo r --release
```

To run portal:

```bash
cd portal
cargo r --release
```

Select the server location (default: `localhost`), then press Enter, to load the client (guest) app.

Note that, without closing the portal, you can recompile the client app, and refresh.
