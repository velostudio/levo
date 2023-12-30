#!/bin/bash

set -e

wit-bindgen c ../../spec

WASI_SDK_PATH=$1

CC="${WASI_SDK_PATH}/bin/clang --sysroot=${WASI_SDK_PATH}/share/wasi-sysroot"
$CC my_world.c  my_world_component_type.o  src/my-component.c -o my-core.wasm -mexec-model=reactor
wasm-tools component new my-core.wasm --adapt ../wasi_snapshot_preview1.reactor.wasm -o main.component.wasm
wasm-tools component wit main.component.wasm
mkdir -p "../../levo-server/public"
cargo run --package brotli-encoder --release -- main.component.wasm "../../levo-server/public/404.wasm"
