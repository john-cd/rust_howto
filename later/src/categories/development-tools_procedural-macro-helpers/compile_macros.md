<!-- TODO Reviewed ??-??-2026 -->

# Compile Macros Ahead of Time

{{#include compile_macros.incl.md}}

## `watt` {#watt}

[![watt][c~watt~docs~badge]][c~watt~docs]{{hi:watt}}
[![watt~crates.io][c~watt~crates.io~badge]][c~watt~crates.io]
[![watt~repo][c~watt~repo~badge]][c~watt~repo]
[![watt~lib.rs][c~watt~lib.rs~badge]][c~watt~lib.rs]
[![cat~development-tools::procedural-macro-helpers][cat~development-tools::procedural-macro-helpers~badge]][cat~development-tools::procedural-macro-helpers]

[`watt`][c~watt~repo] runs procedural macros compiled to WebAssembly. Its documented workflow uses two crates: an implementation crate compiled to `wasm32-unknown-unknown`, and a small `proc-macro` shim that embeds and invokes the resulting `.wasm` file.

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

The Watt repository remains available and documents this workflow. As of 2026-10-25, its releases page lists `0.5.0` as a pre-release and shows a sparse release history. That is not evidence that the project is abandoned; verify Rust edition, `syn`, and `proc-macro2` compatibility against the current repository before adopting it. See the [release history][watt~releases].

## Related Topics {#related-topics .skip}

- [[macros | Code generation and macros]].
- [[development-tools_procedural-macro-helpers | Procedural macro helpers]].
- [[wasm | WebAssembly]].
- [[building | Cargo build configuration]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
