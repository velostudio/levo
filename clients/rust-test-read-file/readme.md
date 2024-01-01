Run from portal directory 

```
cargo r --release -- -a "../clients/rust-test-read-file"
```

then type `localhost/read-file.wasm`.

Inspect logs:

```
[portal/src/main.rs:172] from_wasm = "Hello from Rust! (1280x678)"
[portal/src/main.rs:172] from_wasm = "Hello from hello.txt!\n"
```

