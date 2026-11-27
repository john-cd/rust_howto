<!-- TODO Reviewed ??-??-2026. -->

# Compile Macros Ahead of Time

{{#include compile_macros.incl.md}}

## `watt` {#watt}

{{#badges watt development-tools::procedural-macro-helpers=}}

[`watt`][c~watt~repo] runs procedural macros compiled to WebAssembly. Its documented workflow uses two crates: an implementation crate compiled to `wasm32-unknown-unknown`, and a small `proc-macro` shim that embeds and invokes the resulting `.wasm` file.

The demo wraps an expression in parentheses. Watt loads the guest at compile time, hands it the compiler's input tokens, and returns the output tokens for normal compilation.

### Build and call the guest

The guest uses Watt's pinned `proc-macro2` fork for its token ABI. Keep that patch in the guest workspace so the native procedural-macro examples continue to use the upstream crate.

```rust,noplayground
{{#include ../../../crates/cats/development_tools_procedural_macro_helpers/watt_round_trip/guest/src/lib.rs:example}}
```

The native shim embeds the artifact by absolute path and calls the exported function of the same name.

```rust,noplayground
{{#include ../../../crates/cats/development_tools_procedural_macro_helpers/watt_round_trip/src/lib.rs:example}}
```

From the repository root in PowerShell, rebuild the guest and its standard library for the WebAssembly MVP instruction set. The standard library shipped with current Rust targets newer Wasm features, which Watt 0.5 may reject during decoding.

```powershell
rustup toolchain install nightly-2026-11-16 --profile minimal --component rust-src
$demo = "bk/crates/cats/development_tools_procedural_macro_helpers/watt_round_trip"
$env:CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS = "-C target-cpu=mvp"
cargo +nightly-2026-11-16 build -Z build-std=std,panic_abort --locked --release `
  --target wasm32-unknown-unknown --target-dir target/watt-demo --manifest-path "$demo/guest/Cargo.toml"
$env:WATT_DEMO_WASM = (Resolve-Path target/watt-demo/wasm32-unknown-unknown/release/watt_guest.wasm).Path
cargo test --locked --manifest-path "$demo/Cargo.toml" --target-dir target/watt-demo
```

The test compiles a caller and checks that `2 * parenthesize!(20 + 1)` produces `42`. Without parentheses, the corresponding Rust expression evaluates to `41`.

```rust,noplayground
{{#include ../../../crates/cats/development_tools_procedural_macro_helpers/watt_round_trip/tests/expansion.rs:example}}
```

Treat Watt as an advanced deployment option for macros that work entirely through tokens. Keep the guest, shim, lockfiles, and artifact build together so downstream users can trace the embedded Wasm to its source. Use [`cargo build --timings`][cargo~timings~docs] to compare clean and cached builds before adopting it for performance. The [release history][watt~releases] documents the runtime and token ABI.

## Related Topics {#related-topics .skip}

- [[macros | Code generation and macros]].
- [[development-tools_procedural-macro-helpers | Procedural macro helpers]].
- [[wasm | WebAssembly]].
- [[building | Cargo build configuration]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
