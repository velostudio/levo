# levo-server

levo-server is skeleton for server that response with brotli encoded wasm file.

To run it `cargo r --release`
(add `webtransport` feature if webtransport is used)  

This MVP uses self signed certificate. It responds to a client request with brotli file from `public` directory. 
