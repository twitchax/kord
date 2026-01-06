# Kord Development Guide

## Development Setup

### Testing

```bash
cargo make --no-workspace test
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

### Precision Features

The project supports multiple training and storage precision levels:

**Training Precision** (pick one):
- `ml_train_precision_fp32` (default) - Full 32-bit floating point training
- `ml_train_precision_fp16` - Half precision (16-bit) training with dynamic loss scaling
- `ml_train_precision_bf16` - BFloat16 training with dynamic loss scaling

**Storage Precision** (pick one):
- `ml_store_precision_full` (default) - Store models as full precision
- `ml_store_precision_half` - Store models as half precision (smaller files)

**Note:** NdArray-based training and hyperparameter tuning require `ml_train_precision_fp32`. Inference always runs on the NdArray backend and automatically converts stored values to f32.

### Data Loaders

Pick one loader configuration:
- `ml_loader_note_binned_convolution` - Note-binned with harmonic convolution
- `ml_loader_mel` - Mel-frequency representation
- `ml_loader_frequency` - Raw frequency space
- `ml_loader_frequency_pooled` - Frequency space with pooling

Add `ml_loader_include_deterministic_guess` to augment ML features with deterministic chord detection.

### Target Encodings

**Exactly one must be enabled:**
- `ml_target_full` - Full 128-note encoding
- `ml_target_folded` - Folded octave encoding (12 notes)
- `ml_target_folded_bass` - Folded with separate bass prediction

When using `ml_target_folded_bass`, the bass pitch uses softmax + cross-entropy loss while other notes use binary cross-entropy. Inference decodes bass via argmax to emit a single pitch class.

### Training Details

- Uses cosine-annealed learning rate schedule starting from `TrainConfig.adam_learning_rate`
- Reduced-precision training uses dynamic loss scaling with skip-on-overflow for gradient stability
- Scale growth/backoff happens automatically per training step

## Release and Publishing

### Full Release with Version Bump

```bash
# Step 1: Bump versions and create git tags (does not publish)
cargo make --no-workspace release

# Step 2: Build and publish to crates.io and npm
cargo make --no-workspace publish-all
```

### Publish Without Version Changes

If you've already bumped versions manually or want to republish:

```bash
cargo make --no-workspace publish-all
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
cargo make --no-workspace build-cli
cargo make --no-workspace build-npm
cargo make --no-workspace build-web

# Publish individually
cargo make --no-workspace publish-crates
cargo make --no-workspace publish-npm
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
docker build -f ./docker/Dockerfile -t twitchax/kord-web .
```

Run:
```bash
docker run -it --rm -p 8080:8080 twitchax/kord-web
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