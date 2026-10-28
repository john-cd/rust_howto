<!-- TODO Reviewed ??-??-2026 -->

# WASM

This crate contains code examples for the **WASM** category on crates.io. It demonstrates integration with key ecosystem crates including `any_spawner`, `anyhow`, `blake3`, `clap`, `gloo-net`, `gloo-storage`.

## Examples Covered

- **Interfacing with Javascript** (`examples/interfacing_with_javascript/`): Demonstrates concepts like `js_import`, `js_sys`, `promises`, `serde_wasm_bindgen`, `web_sys`.
- **Leptos Client** (`examples/leptos_client/`): Demonstrates concepts like `gloo_fetch`, `gloo_storage`, `grpc_web_client`, `leptos_router`.
- **WASM Development** (`examples/wasm_development/`): Demonstrates concepts like `wasm`.
- **WASM Standalone Runtimes** (`examples/wasm_standalone_runtimes/`): Demonstrates concepts like `wasmer`, `wasmtime`, `wasmtime_sandbox`.
- **Yew** (`examples/yew/`): Demonstrates concepts like `yew`.

## Usage

To check or run examples in this crate:

```console
cargo check --package wasm --examples
cargo test --package wasm
```

Code examples correspond to chapters in `bk/src/categories/wasm` or related book sections.
