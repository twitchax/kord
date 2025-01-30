# Leptos + Axum + WASI Example

This was generated from the [start-axum](https://github.com/leptos-rs/start-axum) template, and then lightly adapted for use within a `wasip2` context, exposing the proper [`wasi:http`](https://github.com/WebAssembly/wasi-http) exports.

## Preparation

If you don't have `cargo-leptos` installed you can install it with

```bash
cargo install cargo-leptos --locked
```

You will also likely need to add the `wasip2` target to your Rust installation:

```bash
rustup target add wasm32-wasip2
```

## Build

```bash
cargo leptos build --release && LEPTOS_OUTPUT_NAME=kord-web cargo build --lib --release --target wasm32-wasip2 --no-default-features --features ssr
```

The first command (`cargo leptos build`) builds the frontend _and_ backed, with the frontend in `front` + `site`, and the backend in `release` as a native executable.  Really, we could _just_ build the frontend, but this is easier for now.

However, we want the backend to be built with WASI, hence the second command (`cargo build --target wasm32-wasip2`).  This will create a `wasm32-wasip2` binary in `target/wasm32-wasip2/release`.

## Run

```bash
wasmtime serve ./target/wasm32-wasip2/release/kord_web.wasm -S cli
```

Or,

```bash
spin up
```

Naturally, for a real world use case, you'd add logging, telemetry, better config management, secrets, etc.  This is just the barebones to get you started.

## Deviations from the Template

1. The tokio and http features need to be removed from axum. This makes sense, but is totally fine because we don't need axum to serve anything: we just need it to handle routes. For similar reasons, multi-threading needs to be removed from tokio.
2. Possibly because I was lazy and didn't feel like debugging too much, it seems that the axum file server uses spawn_blocking, which fails due to lack of threads in p2. I got around this by just using rust-embed, which is a fun bit of inception because the client wasm is embedded in the server wasm! I could have used spin fileserver, but I wanted a generic implementation.
3. My translation layer between request / response types is pretty rudimentary: just copies method, headers, and body. Not great, but not trying to boil the ocean.

## License

MIT