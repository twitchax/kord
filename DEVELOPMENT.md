# kord development

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
$ cargo wasi build --release --no-default-features --features cli --features ml_infer --features analyze_file
$ cargo wasix publish
```