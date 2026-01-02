# kord development

## Precision Features
- `ml_train_precision_fp32` remains the default; optional `ml_train_precision_fp16` and `ml_train_precision_bf16` enable reduced-precision training.
- NdArray-based training and hyper-parameter tuning still require `ml_train_precision_fp32`.
- Model artifact precision is controlled by `ml_store_precision_full` (default) or `ml_store_precision_half`; inference still runs on the NdArray backend, which converts stored values back into `f32` automatically.
- Reduced-precision training uses dynamic loss scaling (with skip-on-overflow) to keep gradients stable; scale growth/backoff happens automatically per step.
- Training now uses a cosine-annealed learning rate schedule (Burn's `CosineAnnealingLrScheduler`), seeded at `TrainConfig.adam_learning_rate` with the total epoch count.
- When `ml_target_folded_bass` is enabled, the bass pitch head is optimized with softmax + cross-entropy while the remaining note-mask logits keep binary cross-entropy; inference decodes the bass slot via argmax so only one pitch class is emitted.
- Exactly one of `ml_target_full`, `ml_target_folded`, or `ml_target_folded_bass` must be enabled at build time. Pick the encoding you want and disable the others.

## Get libtorch on windows

Download libtorch from https://pytorch.org/get-started/locally/, and make sure to set correct variables: https://github.com/LaurentMazare/tch-rs?tab=readme-ov-file#libtorch-manual-install.

Extract to a directory, e.g. `C:\libtorch`.

```powershell
# Set environment variable for libtorch
# This is required for the build process to find libtorch
$Env:LIBTORCH = "C:\libtorch"
```

## Test

```bash
$ cargo make --no-workspace test
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

## Examples

### Example Train Command

```
cargo run --bin kord --no-default-features --features "cli ml_train ml_tch ml_train_precision_fp32 ml_store_precision_full ml_loader_frequency_pooled ml_target_folded_bass" --release -- -q ml train --backend tch --training-sources samples/captured --training-sources samples/slakh --training-sources sim --noise-asset-root samples/noise --destination model --model-epochs 16
```

### Example Training Check Command

```
cargo check --bin kord --no-default-features --features "cli ml_train ml_train_precision_fp32 ml_store_precision_full ml_tch ml_loader_frequency_pooled ml_target_folded_bass"
```

## TODO

- More epochs?
- Reduce model size with smaller MHA, fewer layers, or dropout changes.
- Hyperparameter tuning should allow all backends.
- Add a synth to the frontend so the sounds are more friendly.