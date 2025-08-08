#!/bin/bash

set -e

cargo leptos build --release && LEPTOS_OUTPUT_NAME=kord-web cargo build --lib --release --target wasm32-wasip2 --no-default-features --features ssr
wasmtime serve ../target/wasm32-wasip2/release/kord_web.wasm -S cli