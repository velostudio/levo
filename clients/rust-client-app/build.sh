#!/bin/bash

set -e

cargo build --target wasm32-wasi --release
wasm-tools component new ../../target/wasm32-wasi/release/rust_client_app.wasm \
  -o my-component.wasm --adapt ../wasi_snapshot_preview1.reactor.wasm
wasm-tools component wit my-component.wasm
mkdir -p "../../levo-server/public"
cargo run --package brotli-encoder --release -- my-component.wasm "../../levo-server/public/rust.wasm"

