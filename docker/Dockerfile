# Top-level container for planner and builder.
FROM lukemathwalker/cargo-chef:latest-rust-latest AS chef

RUN apt-get update && apt-get install -y alsa-tools libasound2-dev
RUN cargo install cargo-leptos wasm-opt

WORKDIR /app

# Copy the toolchain defs in before adding targets so that the right versions get pulled for the targets.
COPY rust-toolchain.toml .
RUN rustup target add wasm32-unknown-unknown wasm32-wasip2

# Run the planner separately.
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build the app.
FROM chef AS builder

# Copy the recipe from the planner stage, but docker will skip if the plan remains the same.
COPY --from=planner /app/recipe.json recipe.json

# Normally, this would be unnecessary to have multiple "cook" commands, but we're doing some weird wasm32 frontend / backend stuff, so we're trying to "cook" as much as we can,
# in the hopes that it save us time when we don;t change dependencies (even though it _is_ overkill for what actually gets built).
RUN cargo chef cook --package kord-web --target-dir=/app/target/front --profile=wasm-release --target wasm32-unknown-unknown --no-default-features --features hydrate --recipe-path recipe.json
RUN cargo chef cook --package kord-web --release --target wasm32-wasip2 --no-default-features --features ssr --recipe-path recipe.json
RUN cargo chef cook --package kord-web --release --no-default-features --features ssr --recipe-path recipe.json

COPY . .

ENV LEPTOS_OUTPUT_NAME=kord-web
RUN cargo leptos build --release
RUN cargo build --package kord-web --lib --release --target wasm32-wasip2 --no-default-features --features ssr

FROM twitchax/wasmtime
COPY --from=builder app/target/wasm32-wasip2/release/kord_web.wasm /app.wasm

ENTRYPOINT [ "wasmtime", "serve", "/app.wasm", "-S", "cli" ]