# Kord — Agents Guide

This document provides detailed workflows and troubleshooting for AI coding agents working in this repository. 

> **Quick Start**: For essential architecture patterns and conventions, see [`.github/copilot-instructions.md`](.github/copilot-instructions.md) first.

This guide focuses on comprehensive build workflows, testing procedures, and detailed troubleshooting steps.

## Workspace Overview
- `kord/` (aka `klib`): Core music theory/audio/ML library and CLI. Pest grammar at `kord/chord.pest`; parser at `kord/src/core/parser.rs`.
- `kord-web/`: Leptos 0.8 SSR app with client hydration (Axum SSR). Also builds to WASI/WASM for edge-like SSR.
  - Loader/target features are forwarded explicitly: default `kord_loader_note_binned` enables `ml_loader_note_binned_convolution` + `ml_target_full` on `klib`. Disable defaults and opt into `kord_loader_frequency` when you need the folded-bass path (`ml_loader_frequency` + `ml_target_folded_bass`).

Key feature flags (core crate):
- Defaults: `default = ["cli", "analyze", "audio"]`
- `cli`: CLI binary features
- `analyze = ["analyze_mic", "analyze_file"]`
- `ml = ["ml_train", "ml_infer"]`, optional `ml_gpu`
- `wasm` / `wasi`
- `audio`

Data/model artifacts used by analysis/ML are in `kord/model/`, `kord/noise/`, and `kord/samples/`.

## Quick Start
Core (library + CLI):
```bash
cargo build -p kord
cargo make --no-workspace test
```
Web (SSR + hydrate):
```bash
cd kord-web
cargo check --features ssr,hydrate
cargo leptos watch
```
WASM (npm package):
```bash
wasm-pack build --features ml_infer --features wasm
```

## Build & Test — Core (Library + CLI)
- Prereqs (Linux): install ALSA headers for audio support
```bash
sudo apt-get update && sudo apt-get install -y libasound2-dev
```
- Build core and CLI:
```bash
cargo build -p kord
```
- Run tests:
```bash
cargo make --no-workspace test
```

### Core Library Patterns
- Types/traits: `Note`, `Chord`, `Modifier`; builder via `Chordable`.
- Parsing: `Parsable` with Pest grammar in `kord/chord.pest` and logic in `kord/src/core/parser.rs`.
- Errors: `anyhow::Result<T>` aliased as `Res<T>`; use for fallible functions.
- Statics: prefer `LazyLock` for computed tables (see `core/modifier.rs`).
- Audio analysis: normalized FFT space (8192 bins to C9).
- ML: config via `TrainConfig`; constants in `kord/src/ml/base/mod.rs` (e.g., `FREQUENCY_SPACE_SIZE`).

## Build & Run — Web App (SSR + Hydrate)
From `kord-web/`:
- Developer server with SSR + client hydrate:
```bash
cargo leptos watch
```
- Type-check with SSR + Hydrate features (useful for CI/agents):
```bash
cargo check --features ssr,hydrate
```
- Release build (SSR binary + client assets):
```bash
cargo leptos build --release
```

### Web App Patterns (Leptos + Thaw)
- SSR/hydrate: the lib builds with `hydrate`; the bin builds with `ssr`. Keep parity across both features.
- App shell: wrap with `thaw::ssr::SSRMountStyleProvider` and `thaw::ConfigProvider` (custom theme) in `kord-web/src/app/mod.rs`.
- Callbacks: Thaw expects `on_click: Option<BoxOneCallback<MouseEvent>>`. Use `thaw_utils` helpers and shared wrappers.
- Inputs: bind `thaw::Input` with `RwSignal<String>`.
- Layout/typography: use `Space`/`Flex`; `Text` with `TextTag` for headings.
- Forms: prefer `Field label="…"` wrapping `Input` instead of raw labels.
- Timing: use `leptos-use` utilities (e.g., `use_timestamp`) over manual intervals.
- Shared UI: reuse components in `kord-web/src/app/shared.rs` for consistent Thaw integration.

### WASI SSR Library (for Wasmtime/Wasmer)
- Build SSR library targeting `wasm32-wasip2`:
```bash
export LEPTOS_OUTPUT_NAME=kord-web
cargo build --lib --release --target wasm32-wasip2 --no-default-features --features ssr -p kord-web
```
- Run with Wasmtime (example):
```bash
wasmtime serve ./target/wasm32-wasip2/release/kord_web.wasm -S cli
```

### NPM/WASM Library Build
- Build the WASM package exposing the core API for JS:
```bash
wasm-pack build --features ml_infer --features wasm
```
- Publish flow (summary): rename package to `kordweb`, then `wasm-pack publish`.

### Wasmer Binary (reduced capabilities)
- Build a WASI binary with limited features:
```bash
cargo wasi build --release --no-default-features \
  --features wasi --features cli --features ml_infer --features analyze_file -p kord
```

## Editing Grammar & Parser
- Chord grammar lives in `kord/chord.pest`.
- Parsing implementation is in `kord/src/core/parser.rs`.
- When extending grammar:
  - Update both the `.pest` rules and corresponding parser code.
  - Add targeted tests under `kord/src/**` or workspace tests under `kord/tests/**`.
  - Keep backward compatibility for existing chord strings when possible.

## Conventions for Agents
- Keep changes minimal and focused; avoid unrelated refactors.
- Respect feature gating patterns (`#[cfg(feature = "…")]`) and keep SSR/hydrate parity for the web app.
- Prefer fixing root causes over surface workarounds; don’t change public APIs unless required.
- Follow existing style; don’t add license headers; avoid one-letter variable names.
- Web UI (Leptos + Thaw):
  - Use `thaw_utils` callback helpers (`BoxOneCallback`/`ArcOneCallback`) per existing components.
  - Bind `thaw::Input` values via `RwSignal<String>` (Model<String>).
  - Use shared wrappers from `kord-web/src/app/shared.rs` for consistent UI.
- When editing code, validate with targeted checks:
  - Core: `cargo make --no-workspace test`
  - Web: `cargo check -p kord-web --features ssr,hydrate`
  - Full workspace checks are good before PRs: `cargo make --no-workspace test` + `cargo check`.

### SSR/Hydrate Gotchas
- Prefer pointer events for cross-input support; ensure release/cancel stops any audio.
- Gate browser-only code with `#[cfg(feature = "hydrate")]` and provide server fallbacks.
- Keep DOM-dependent code out of SSR paths; use feature flags or runtime checks.

## Quick Tasks Reference
- Describe a chord (CLI):
```bash
kord describe Cmaj7
```
- Guess from audio (deterministic vs ML):
```bash
kord analyze mic
kord ml infer mic
```

## Testing — Web
- Run library tests for the core crate as usual.
- For the web crate, prefer type-checking both SSR and hydrate features:
```bash
cd kord-web
cargo check --features ssr,hydrate
```
- Integration tests are run as part of the main test suite:
```bash
cargo make --no-workspace test
```

## Troubleshooting Notes
- If audio-related crates fail to build on Linux, ensure `libasound2-dev` is installed.
- For proc-macro ABI cache issues during development, a clean rebuild can help:
```bash
cargo clean -p kord-web && cargo check -p kord-web --features ssr,hydrate
```
 - If `cargo leptos` is missing, install it:
```bash
cargo install cargo-leptos
```
 - WASM/WASI tooling often needed for advanced flows:
```bash
cargo install cargo-wasi      # for cargo wasi
cargo install wasm-pack       # for npm/wasm builds
brew install wasmtime || sudo apt-get install wasmtime
brew install wasmer   || curl https://get.wasmer.io -sSfL | sh
```

---
This guide consolidates details from the top-level README and DEVELOPMENT notes to make automated and human contributions faster and safer. If something seems off or you need more detail, check `README.md` and `DEVELOPMENT.md` for the authoritative context.
