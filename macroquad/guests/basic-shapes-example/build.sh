#!/bin/bash

set -e

cargo build --target wasm32-wasi --release
wasm-tools component new ../../../target/wasm32-wasi/release/macroquad_basic_shapes.wasm \
  -o my-component.wasm --adapt ../wasi_snapshot_preview1.reactor.wasm
wasm-tools component wit my-component.wasm
cp -vf my-component.wasm ../../macroquad.wasm 
