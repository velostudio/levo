# levo-server

levo-server is skeleton for server that response with brotli encoded wasm file.

To run it `cargo r --release`
(add `webtransport` feature for enabling webtransport protocol)  

Server responds to a client request with brotli file from `public` directory. 
