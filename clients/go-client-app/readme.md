# Example of Go client

Get `wit-bindgen-cli`:

```
cargo install --git https://github.com/bytecodealliance/wit-bindgen wit-bindgen-cli
```

Install [`Go`](https://go.dev/doc/install).

Install [`TinyGo`](https://tinygo.org/getting-started/install/).

Generate Go bindings for [`../../spec/host.wit`](../../spec/host.wit) using [`./bindgen.sh`](./bindgen.sh) (or copy the pre-generated bindings in [`./my-world/`](./my-world/))

Write guest code in [`./src/my-component.go`](./src/my-component.go)

Run [`./build.sh`](./build.sh)
