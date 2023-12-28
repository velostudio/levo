#!/bin/bash

set -e

cd client-app

cargo build --target wasm32-wasi --release

wasm-tools component new ../target/wasm32-wasi/release/client_app.wasm \
  -o my-component.wasm --adapt ./wasi_snapshot_preview1.reactor.wasm
wasm-tools component wit my-component.wasm

cd ../brotli-encoder
cargo run --release "../client-app/my-component.wasm" "../levo-server/public/rust.wasm"
