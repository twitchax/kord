# Kord AI Coding Agent Instructions

Kord is a music theory library and CLI tool with ML capabilities, built in Rust with multi-platform support (native, WASM, WASI).

## Architecture Overview

**Workspace Structure**: Cargo workspace with two main crates:
- `kord/`: Core library (`klib`) + CLI binary with music theory, audio analysis, and ML training
- `kord-web/`: Leptos + Axum web app running as WASI service with embedded frontend

**Core Library Design**: Music theory primitives built around `Note`, `Chord`, `Modifier` traits with extensive parsing via Pest grammar (`chord.pest`). The `Chordable` trait provides builder pattern for chord construction.

**Feature Flag Architecture**: Heavily feature-gated for different deployment targets:
- `cli`: CLI functionality (required for binary)  
- `analyze = ["analyze_mic", "analyze_file"]`: Audio processing with FFT/spectrum analysis
- `ml = ["ml_train", "ml_infer"]`: Machine learning with Burn framework  
- `ml_gpu`: GPU acceleration via burn-tch/burn-wgpu
- `wasm`/`wasi`: WebAssembly compilation targets
- `audio`: Audio playback via rodio

## Key Development Workflows

**Platform-Specific Builds**:
```bash
# Native CLI (default features)
cargo build

# Web frontend + backend
cd kord-web && cargo leptos build --release
LEPTOS_OUTPUT_NAME=kord-web cargo build --lib --release --target wasm32-wasip2 --no-default-features --features ssr

# WASI binary for Wasmer
cargo wasi build --release --no-default-features --features wasi --features cli --features ml_infer --features analyze_file

# WASM for NPM
wasm-pack build --features ml_infer --features wasm
```

**Linux Dependencies**: Always install `libasound2-dev` for ALSA support before building (see CI workflow).

**ML Training Pipeline**: Uses Burn framework with configurable backends (CPU/GPU). Training config in `TrainConfig` struct controls hyperparameters, data simulation, and model architecture.

## Project-Specific Patterns

**Error Handling**: Uses `anyhow::Result` aliased as `Res<T>` and `Void` for `Result<(), Error>` throughout.

**Static Data**: Extensive use of `LazyLock` for computed static arrays (e.g., `ALL_PITCHES`, `KNOWN_MODIFIER_SETS`). See `kord/src/core/modifier.rs` for patterns.

**Parser Architecture**: Pest grammar in `chord.pest` with hand-written parser logic in `core/parser.rs`. Follow existing patterns for extending chord notation.

**Multi-Target Compilation**: Heavy use of `#[cfg(feature = "...")]` guards. New features should follow the hierarchical feature flag pattern (base → specific implementations).

**Audio Processing**: FFT-based analysis in `analyze/` module. Frequency space standardized at 8192 bins covering up to C9.

## Integration Points

**Frontend-Backend**: Leptos SSR with rust-embed for static assets. Backend runs as WASI service with custom request/response translation layer.

**ML Pipeline**: Binary samples stored in `samples/` and `noise/` directories. Training uses `KordItem` structs with frequency space + label format.

**Build System**: Custom `build.rs` sets platform-specific cfg flags. Uses workspace profiles including `wasm-release` for optimized WASM builds.

**External Dependencies**: Rodio for audio, Symphonia for file formats, Burn for ML, Leptos for web, Pest for parsing.

## Critical Conventions

- Use `#[coverage(off)]` for non-testable code (UI, training loops)
- ML constants in `ml/base/mod.rs` (FREQUENCY_SPACE_SIZE, NUM_CLASSES, etc.)
- Follow existing trait patterns for music theory types (HasStaticName, Parsable, etc.)
- Feature flags control compilation - always check dependencies when adding new functionality
