############################
# Builder (dependency graph)
############################
FROM lukemathwalker/cargo-chef:latest-rust-latest AS chef

RUN apt-get update && apt-get install -y --no-install-recommends \
	alsa-tools libasound2-dev pkg-config && \
	rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-leptos

WORKDIR /app
COPY rust-toolchain.toml .

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

############################
# Build stage
############################
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Copy the full source and build only the server-side rendered binary.
COPY . .
ENV LEPTOS_OUTPUT_NAME=kord-web
RUN cargo leptos build --release --bin kord-web --features ssr

############################
# Runtime (slim)
############################
FROM debian:stable-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends libasound2 ca-certificates && \
	rm -rf /var/lib/apt/lists/*
WORKDIR /app

# Copy the binary produced by cargo-leptos (server binary lives in target/release)
COPY --from=builder /app/target/release/kord-web /usr/local/bin/kord-web
COPY --from=builder /app/target/site /app/site

ENV LEPTOS_SITE_ADDR=0.0.0.0:3000 \
	RUST_LOG=info

EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/kord-web"]