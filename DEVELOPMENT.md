# kord development

## Get libtorch on windows

Download libtorch from https://pytorch.org/get-started/locally/, and make sure to set correct variables: https://github.com/LaurentMazare/tch-rs?tab=readme-ov-file#libtorch-manual-install.

Extract to a directory, e.g. `C:\libtorch`.

```powershell
# Set environment variable for libtorch
# This is required for the build process to find libtorch
$Env:LIBTORCH = "C:\libtorch"
```

## Publish to Cargo

```bash
$ cargo publish
```

## Publish to NPM

```bash
$ wasm-pack build --features ml_infer --features wasm
```

Rename package to `kordweb`,

```bash
$ wasm-pack publish
```

## Publish to wasmer

```bash
$ cargo wasi build --release --no-default-features --features wasi --features cli --features ml_infer --features analyze_file
$ wasmer publish
```

# Web WASM

## Build

```bash
$ export LEPTOS_OUTPUT_NAME=kord-web
$ cargo leptos build --release
$ cargo build --package kord-web --lib --release --target wasm32-wasip2 --no-default-features --features ssr
```

## Run

```bash
$ wasmtime serve ./target/wasm32-wasip2/release/kord_web.wasm -S cli
```

# Web Docker

## Build

```bash
docker build -f ./docker/Dockerfile -t twitchax/kord-web .
```

## Run

```bash
docker run -it --rm -p 8080:8080 twitchax/kord-web
```