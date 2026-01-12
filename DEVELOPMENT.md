# Kord Development Guide

## Development Setup

### Testing

```bash
cargo make test
```

### Platform-Specific Setup: libtorch on Windows

If you need GPU training support with the `tch` backend on Windows, you'll need to install libtorch manually:

1. Download libtorch from https://pytorch.org/get-started/locally/
2. Follow the setup instructions at https://github.com/LaurentMazare/tch-rs?tab=readme-ov-file#libtorch-manual-install
3. Extract to a directory (e.g., `C:\libtorch`)
4. Set the environment variable:

```powershell
$Env:LIBTORCH = "C:\libtorch"
```

## ML Training Configuration

The ML pipeline exposes toggles that change both the training inputs and labels without modifying source code. Loader features and the deterministic guess flag can be combined, while target encodings are mutually exclusive so that model layouts stay predictable.

### Training Precision Features

The project supports multiple training precision levels. **Choose exactly one:**

| Feature                      | Description                                                           | Notes                                              |
| ---------------------------- | --------------------------------------------------------------------- | -------------------------------------------------- |
| `ml_train_precision_fp32`    | Full 32-bit floating point training                                   | Default; required for NdArray backend and HPT      |
| `ml_train_precision_fp16`    | Half precision (16-bit) training with dynamic loss scaling            | Requires compatible backend (e.g., tch with CUDA)  |
| `ml_train_precision_bf16`    | BFloat16 training with dynamic loss scaling                           | Requires compatible backend (e.g., tch with CUDA)  |

**Note:** NdArray-based training and hyperparameter tuning require `ml_train_precision_fp32`. Inference always runs on the NdArray backend and automatically converts stored values to f32.

### Storage Precision Features

**Choose exactly one:**

| Feature                      | Description                                                           |
| ---------------------------- | --------------------------------------------------------------------- |
| `ml_store_precision_full`    | Store models as full precision                                        |
| `ml_store_precision_half`    | Store models as half precision (smaller files)                        |

### Sample Loader Features

**Choose exactly one:**

| Feature                             | Description                                                    | Input width (before deterministic guess) |
| ----------------------------------- | -------------------------------------------------------------- | ---------------------------------------- |
| `ml_loader_note_binned_convolution` | Uses the existing note-binned harmonic convolution (128 bins)  | 128                                      |
| `ml_loader_mel`                     | Applies mel filter banks to the full spectrum (512 bands)      | 512                                      |
| `ml_loader_frequency`               | Feeds the raw 8,192-bin frequency spectrum                     | 8192                                     |
| `ml_loader_frequency_pooled`        | Averages the raw spectrum into 2,048 pooled bins (factor ×4)   | 2048                                     |

**Optional add-on:**

| Feature                                 | Description                                                                                                                                  |
| --------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| `ml_loader_include_deterministic_guess` | Prepends the deterministic 128-note guess vector to whichever loader you selected above (doubling 128-bin inputs, adding 128 to the others) |

### Target Encoding Features

**Choose exactly one:**

| Feature                 | Description                                                                                                                                                           | Output width contribution |
| ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------- |
| `ml_target_full`        | Emits the full 128-note mask (per MIDI note across octaves)                                                                                                           | +128                      |
| `ml_target_folded`      | Emits a folded 12-class pitch-class mask (one octave)                                                                                                                 | +12                       |
| `ml_target_folded_bass` | Emits two 12-class masks: a categorical bass pitch class (trained with softmax / cross-entropy) and a multi-hot mask of every pitch class present across all octaves | +24                       |

When using `ml_target_folded_bass`, the bass pitch uses softmax + cross-entropy loss while other notes use binary cross-entropy. Inference decodes bass via argmax to emit a single pitch class.

### Example ML Feature Configurations

```bash
# Default (note-binned + deterministic guess, 128-note target)
cargo check

# Mel features with deterministic guess and folded targets
cargo check --no-default-features \
   --features "cli ml_infer ml_loader_mel ml_loader_include_deterministic_guess ml_target_folded"

# Raw frequency spectrum without deterministic guess, folded targets only
cargo check --no-default-features \
   --features "cli ml_infer ml_loader_frequency ml_target_folded"

# Pooled raw spectrum with deterministic guess, folded targets only
cargo check --no-default-features \
   --features "cli ml_infer ml_loader_frequency_pooled ml_loader_include_deterministic_guess ml_target_folded"

# Pooled spectrum with deterministic guess and folded+bass targets
cargo check --no-default-features \
   --features "cli ml_infer ml_loader_frequency_pooled ml_loader_include_deterministic_guess ml_target_folded_bass"
```

> Make sure exactly one loader feature is enabled at a time, and exactly one target feature is enabled overall. The deterministic guess flag can be toggled independently to suit experiments.

### Training Details

- Uses cosine-annealed learning rate schedule starting from `TrainConfig.adam_learning_rate`
- Reduced-precision training uses dynamic loss scaling with skip-on-overflow for gradient stability
- Scale growth/backoff happens automatically per training step

## Release and Publishing

### Full Release with Version Bump

```bash
# Step 1: Bump versions and create git tags (does not publish)
cargo make release

# Step 2: Build and publish to crates.io and npm
cargo make publish-all
```

### Publish Without Version Changes

If you've already bumped versions manually or want to republish:

```bash
cargo make publish-all
```

**This orchestrates:**
1. Format check and tests (`check-all`)
2. Build CLI binary (`build-cli`)
3. Build WASM package (`build-npm`)
4. Build Leptos web app (`build-web`)
5. Publish `kord` crate to crates.io (`publish-crates`)
6. Rename npm package to `kordweb` and publish (`publish-npm`)

### Individual Tasks

```bash
# Build components individually
cargo make build-cli
cargo make build-npm
cargo make build-web

# Publish individually
cargo make publish-crates
cargo make publish-npm
```

### Publish to Wasmer

```bash
cargo wasi build --release --no-default-features \
  --features wasi --features cli --features ml_infer --features analyze_file
wasmer publish
```

## Web Deployment

### Docker

Build:
```bash
cargo make docker-build
```

Run:
```bash
cargo make docker-run
```

### Fly.io

Deploy:
```bash
cargo make fly-deploy
```

## Training Examples

### Example Training Command

```bash
cargo run --bin kord --no-default-features \
  --features "cli ml_train ml_tch ml_train_precision_fp32 ml_store_precision_full ml_loader_mel ml_target_folded" \
  --release -- -q ml train \
  --backend tch \
  --training-sources samples/captured \
  --training-sources samples/slakh \
  --training-sources sim \
  --noise-asset-root samples/noise \
  --destination model \
  --model-epochs 16
```

### Type-Check Training Configuration

```bash
cargo check --bin kord --no-default-features \
  --features "cli ml_train ml_train_precision_fp32 ml_store_precision_full ml_tch ml_loader_mel ml_target_folded"
```

## TODO

- Evaluate increasing training epochs
- Reduce model size (smaller MHA, fewer layers, dropout tuning)
- Enable hyperparameter tuning with all backends (currently requires NdArray)
- Add synthesizer to frontend for more pleasant audio feedback