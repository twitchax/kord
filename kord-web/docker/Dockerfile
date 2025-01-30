FROM twitchax/ubuntu

RUN curl https://wasmtime.dev/install.sh -sSf | bash
COPY target/wasm32-wasip2/release/kord_web.wasm /app.wasm

CMD ["/root/.wasmtime/bin/wasmtime", "serve", "/app.wasm", "-S", "cli"]