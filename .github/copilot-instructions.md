# Kord AI Coding Agent Instructions

Kord is a Rust workspace with a music theory/ML core and a Leptos web app. This guide captures the minimum you need to be productive here.

## Workspace Overview
- `kord/` (aka `klib`): core library + CLI (music theory, audio analysis, ML). Pest grammar in `kord/chord.pest`; parser in `kord/src/core/parser.rs`.
- `kord-web/`: Leptos 0.8 SSR app (Axum for SSR) + client hydrate; also builds to WASI/WASM. UI uses Thaw (pre-release) components.
- Feature flags (core): `cli`, `analyze=[analyze_mic,analyze_file]`, `ml=[ml_train,ml_infer]`, `ml_gpu`, `wasm`/`wasi`, `audio`.

## Build & Run
- Core/CLI dev: `cargo build` | tests: `cargo test`. On Linux, install `libasound2-dev` for audio.
- Web dev (SSR + hydrate):
	- `cd kord-web`
	- Dev: `cargo leptos watch`
	- Release: `cargo leptos build --release`
- WASI SSR lib (for Wasmer):
	- `LEPTOS_OUTPUT_NAME=kord-web cargo build --lib --release --target wasm32-wasip2 --no-default-features --features ssr`
- NPM WASM (library): `wasm-pack build --features ml_infer --features wasm`
- Wasmer binary (reduced capabilities): `cargo wasi build --release --no-default-features --features wasi --features cli --features ml_infer --features analyze_file`

## Core Library Patterns (kord)
- Types/traits: `Note`, `Chord`, `Modifier`, builder pattern via `Chordable`. Parsing via `Parsable` and Pest grammar.
- Error/result: `anyhow::Result` aliased to `Res<T>`; `Void` for `Result<(), Error>`.
- Statics: `LazyLock` for computed tables (see `kord/src/core/modifier.rs`).
- ML: constants in `kord/src/ml/base/mod.rs` (e.g., `FREQUENCY_SPACE_SIZE`), training configured by `TrainConfig`.
- Audio analysis: FFT with normalized frequency space (8192 bins up to C9).

## Web App Patterns (kord-web)
- Features: lib builds with `hydrate`; bin with `ssr`. See `[package.metadata.leptos]` for default features/profile.
- App shell: wrap with `thaw::ssr::SSRMountStyleProvider` and `thaw::ConfigProvider` (custom theme) in `kord-web/src/app/mod.rs`.
- UI library: Thaw components. Important specifics:
	- `Input`: bind `value` to `RwSignal<String>` (Model<String>). Use `InputSuffix`/`InputPrefix` slots for adornments.
	- Callbacks: Thaw expects `on_click: Option<BoxOneCallback<MouseEvent>>`. Use `thaw_utils::BoxOneCallback` and accept `impl Into<BoxOneCallback<_>>` in wrappers.
	- Layout/typography: use `Space`/`Flex` for spacing; `Text` with `TextTag` for headings.
	- Forms: prefer `Field label="…"` wrapping `Input` instead of raw labels.
	- Timing: use `leptos-use` (e.g., `use_timestamp`) instead of manual intervals for progress.
- JS interop: `kord-web/src/mic.rs` bridges to a JS `recordMicrophone(seconds)` function via `wasm-bindgen`, returns mono Float32 PCM converted to little-endian `Vec<u8>`.
- Shared UI: `kord-web/src/app/shared.rs` exposes `PageTitle`, `PrimaryButton`, `SecondaryButton`, etc., wrapping Thaw primitives for consistency.

## Conventions & Organization
- Heavy use of `#[cfg(feature = "…")]` across crates; follow existing feature gating patterns (base → specific).
- Use `#[coverage(off)]` for code that can’t be tested (UI, long training loops).
- Data/ML artifacts: `kord/model/`, `kord/noise/`, `kord/samples/` contain binaries used by analysis/ML; don’t rename without adjusting code.
- Adversarial pass: always self-review changes for missed abstractions and edge cases; verify SSR/hydrate parity, feature flags, Thaw callback types (`thaw_utils::BoxOneCallback`), and `Input` model binding (`RwSignal<String>`).

## Where to Change Things (examples)
- Extend chord grammar: edit `kord/chord.pest` and parser in `kord/src/core/parser.rs` (mirror existing rule patterns and tests).
- Add a web page: place a component in `kord-web/src/app/*.rs`, add to routes in `mod.rs`, use Thaw `PageTitle`, `Field` + `Input` and Thaw buttons.
- Wrap a Thaw button in shared UI:
	```rust
	use thaw_utils::BoxOneCallback;
	#[component]
	fn PrimaryButton<OC>(on_click: OC, children: Children) -> impl IntoView
	where OC: Into<BoxOneCallback<leptos::ev::MouseEvent>> {
			thaw::Button(appearance=thaw::ButtonAppearance::Primary, on_click=on_click.into())(children)
	}
	```

If any section feels incomplete or you need deeper examples (e.g., Thaw SSR setup, ML training flows), let me know and I’ll expand it.
