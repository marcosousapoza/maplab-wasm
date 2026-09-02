# Maplab WASM

Rust/WebAssembly geospatial calculations for the Maplab browser client.

## Requirements

- Rust 1.97 or newer
- `wasm32-unknown-unknown` target
- wasm-pack 0.15 or newer

## Build

```bash
cargo test
cargo clippy --all-targets -- -D warnings
wasm-pack build --target web --out-dir pkg
```

The generated `pkg/` directory is an npm-compatible artifact and is not committed.
It exports `haversine_distance_km`. Invalid coordinates return JavaScript `NaN`.

The environment is successfully configured when native tests pass and wasm-pack
produces `pkg/maplab_wasm.js`, `pkg/maplab_wasm.d.ts`, and a `.wasm` binary.

## Ownership

This repository owns the generated JavaScript/WASM contract. Report problems owned
by another component in that component's GitHub repository.
