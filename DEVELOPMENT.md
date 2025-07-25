# kord development

## Get libtorch on windows

You need to set the right environment variable so `tch-rs` can find the libtorch library, and download it.

The one that works best right now is `12.8`.

```bash
export TORCH_CUDA_VERSION="cu128"
```

This will automatically download the right binaries, but you may need to turn off any VPNs you are using.

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