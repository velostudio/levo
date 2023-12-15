#!/bin/bash

cd client-app

cargo build --target wasm32-wasi --release

wasm-tools component new ./target/wasm32-wasi/release/wit_play.wasm \
  -o my-component.wasm --adapt ./wasi_snapshot_preview1.reactor.wasm

cd ../brotli-encoder
cargo run --release "../client-app/my-component.wasm" "../levo-server/my-component-wasm.br"
