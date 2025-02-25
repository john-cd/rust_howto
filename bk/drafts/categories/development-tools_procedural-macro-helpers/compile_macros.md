# Compile macros ahead of time

{{#include compile_macros.incl.md}}

## `watt` {#watt}

[![watt][c-watt-badge]][c-watt]{{hi:watt}}
[![watt-crates.io][c-watt-crates.io-badge]][c-watt-crates.io]
[![watt-github][c-watt-github-badge]][c-watt-github]
[![watt-lib.rs][c-watt-lib.rs-badge]][c-watt-lib.rs]
[![cat-development-tools::procedural-macro-helpers][cat-development-tools::procedural-macro-helpers-badge]][cat-development-tools::procedural-macro-helpers]

[`watt`][c-watt]⮳{{hi:watt}} is a runtime for executing Rust procedural [macros][p-macros] compiled as WebAssembly (WASM). By compiling [macros][p-macros] ahead-of-time to WASM, we save all downstream users of the macro from having to compile the macro logic or its dependencies themselves. Instead, what they compile is a small self-contained WASM runtime (~3 seconds, shared by all macros) and a tiny proc macro shim for each macro crate to hand off WASM bytecode into the Watt runtime (~0.3 seconds per proc-macro crate you depend on). This is much less than the 20+ seconds it can take to compile complex procedural [macros][p-macros] and their dependencies.

```rust,editable
{{#include ../../../crates/cats/development_tools_procedural_macro_helpers/tests/watt.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[compile_macros: write (P2)](https://github.com/john-cd/rust_howto/issues/327)

</div>
