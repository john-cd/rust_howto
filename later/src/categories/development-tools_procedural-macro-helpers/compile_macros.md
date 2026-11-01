<!-- TODO Reviewed ??-??-2026 -->

# Compile Macros Ahead of Time

{{#include compile_macros.incl.md}}

## `watt` {#watt}

{{#badges watt development-tools::procedural-macro-helpers=}}

[`watt`][c~watt~repo] runs procedural macros compiled to WebAssembly. Its documented workflow uses two crates: an implementation crate compiled to `wasm32-unknown-unknown`, and a small `proc-macro` shim that embeds and invokes the resulting `.wasm` file.

The following compiled helper demonstrates the token transformation that a Wasm implementation could call. It uses `proc_macro2` so the transformation can run and be tested outside the compiler; the example does not build the Wasm artifact or the Watt shim.

```rust,editable
{{#include ../../../crates/cats/development_tools_procedural_macro_helpers/examples/compile_macros/watt.rs:example}}
```

1. Implement and test the macro normally. For the WebAssembly crate, expose each macro as an `extern "C"` function taking and returning `proc_macro2::TokenStream`, set `[lib] crate-type = ["cdylib"]`, and apply Watt's documented `proc-macro2` patch.
2. Build the implementation for the Wasm target:

   ```sh
   rustup target add wasm32-unknown-unknown
   cargo build --release --target wasm32-unknown-unknown --manifest-path path/to/macro_impl/Cargo.toml
   ```

3. Copy `target/wasm32-unknown-unknown/release/macro_impl.wasm` into the shim crate. Add `watt` to the shim and define a normal procedural-macro entry point:

   ```rust
   use proc_macro::TokenStream;
   use watt::WasmMacro;

   static WASM: &[u8] = include_bytes!("macro_impl.wasm");
   static MACRO: WasmMacro = WasmMacro::new(WASM);

   #[proc_macro]
   pub fn the_macro(input: TokenStream) -> TokenStream {
       MACRO.proc_macro("the_macro", input)
   }
   ```

The `proc-macro2` patch and artifact-copy step are part of the build workflow. Pin and review the implementation and shim dependencies together, and make the Wasm build reproducible in CI before publishing. Do not assume that moving compilation to Wasm improves a particular workspace's build time: compare clean and cached builds with [`cargo build --timings`][cargo~timings~docs] and consider [`sccache`][sccache~repo] first.

The Watt README describes faster downstream compilation as a goal and notes that Wasm execution can add macro-expansion overhead. As of 2026-10-26, its releases page lists `0.5.0` as a pre-release; the release notes include `syn` 2 support. Review the implementation and shim dependencies together before adopting it. See the [Watt repository][c~watt~repo] and [release history][watt~releases].

## Related Topics {#related-topics .skip}

- [[macros | Code generation and macros]].
- [[development-tools_procedural-macro-helpers | Procedural macro helpers]].
- [[wasm | WebAssembly]].
- [[building | Cargo build configuration]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
