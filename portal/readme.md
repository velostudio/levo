# portal

This is starter for client app. Currently it connects to the server using HTTP3 WebTransport protocol, receives brotli encoded wasm file, decodes it, executes it using `wit` definition in `./wit` folder. It provides host implementation of imported functions for wasm execution.  

Simply run it `cargo r --release` with running `levo-server` app.

To enable key logging for inspecting packets in Wireshark run it like so:

```bash
SSLKEYLOGFILE=<PATH_TO_KEY_FILE> cargo r --release
```
(add `webtransport` feature if webtransport is used)
