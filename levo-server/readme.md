# levo-server

levo-server is skeleton for server that response with brotli encoded wasm file.

To run it `cargo r --release`

This MVP uses self signed certificate. It responds to a client request with brotli file from `levo-server` directory. To re-generate brotli file using wasm file uncomment:

```
		     // let mut compressor = CompressorWriter::new(Vec::new()); // write to memory
             // let _ = compressor.write_all(wasm_content.as_slice());
		     // let data = compressor.into_inner().unwrap();
		     // std::fs::write("./my-component-wasm.br", &data);
```

Alternatively, there should be CLI that encodes to brotli.
