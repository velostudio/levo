#!/bin/bash

set -e

export GO111MODULE=on
wit-bindgen tiny-go ../../spec --out-dir=my-world
tinygo build -target=wasi -o main.wasm my-component.go
wasm-tools component embed --world my-world ../../spec main.wasm -o main.embed.wasm
wasm-tools component new main.embed.wasm --adapt ../wasi_snapshot_preview1.reactor.wasm -o main.component.wasm
wasm-tools component wit main.component.wasm

cargo run --package brotli-encoder --release -- main.component.wasm "../../levo-server/public/go.wasm"
