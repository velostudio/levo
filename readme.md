_WIP_

<img src="https://raw.githubusercontent.com/velostudio/levo/main/levo.png" width="128" />

https://staffengineer.github.io/blog/levo.html

## Levo: the good parts

To re-generate brotli encoded wasm file of rust client:

```bash
cd clients/rust-client-app
./build.sh
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

Select the resource location (default: `localhost/rust.wasm`), then press Enter, to load the client (guest) app.

Note that, without closing the portal, you can recompile the client app, and refresh.
