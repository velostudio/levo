Run from portal directory 

```
cargo r --release -- -a "../clients/rust-test-read-file/test-folder/public"
```

then type `localhost/read-file.wasm`.

Inspect logs:

```
[portal/src/main.rs:172] from_wasm = "Hello from Rust! (1280x678)"
[portal/src/main.rs:172] from_wasm = "Hello from public!\n"
Path is not within allowed directory. Allowed: /Users/dmytro.rets/src/new/levo/clients/rust-test-read-file/test-folder/public. Path: /Users/dmytro.rets/src/new/levo/clients/rust-test-read-file/test-folder/private/hello.txt
[portal/src/main.rs:172] from_wasm = "Failed to read private hello.txt"
```


