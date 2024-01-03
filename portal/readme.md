# portal

This is starter for client app. Currently it connects to the server using HTTP3 WebTransport protocol, receives brotli encoded wasm file, decodes it, executes it using `wit` definition in `./wit` folder. It provides host implementation of imported functions for wasm execution.  

Simply run it `cargo r --release` with running `levo-server` app.

To enable key logging for inspecting packets in Wireshark run it like so:

```
SSLKEYLOGFILE=<PATH_TO_KEY_FILE> cargo r --release --feature "no_cert_validation webtransport"
```

(only add `no_cert_validation` feature if `levo-server` is used locally)  
(only add `webtransport` feature if webtransport server is used (such as `levo-server`))
