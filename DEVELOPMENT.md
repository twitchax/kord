# kord development

## Get libtorch on windows

You need to set the right environment variable so `tch-rs` can find the libtorch library.

The one that works best right now is `11.8`.

```bash
export TORCH_CUDA_VERSION="cu118"
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
$ cargo wasi build --release --no-default-features --features cli --features ml_infer --features analyze_file
$ cargo wasix publish
```