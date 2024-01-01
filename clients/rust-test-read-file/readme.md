Run from portal directory 

```
cargo r --release -- -a "../clients/rust-test-read-file/test-folder/public"
```

then type `localhost/read-file.wasm`.

Inspect logs that "guest" app can read from public directory but not from private.
