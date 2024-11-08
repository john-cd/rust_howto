# Compile macros ahead of time

[![watt][c-watt-badge]][c-watt]{{hi:watt}}
[![watt-crates.io][c-watt-crates.io-badge]][c-watt-crates.io]
[![watt-github][c-watt-github-badge]][c-watt-github]
[![watt-lib.rs][c-watt-lib.rs-badge]][c-watt-lib.rs]

Runtime for executing Rust procedural macros compiled as WebAssembly

By compiling macros ahead-of-time to Wasm, we save all downstream users of the macro from having to compile the macro logic or its dependencies themselves.

Instead, what they compile is a small self-contained Wasm runtime (~3 seconds, shared by all macros) and a tiny proc macro shim for each macro crate to hand off Wasm bytecode into the Watt runtime (~0.3 seconds per proc-macro crate you depend on). This is much less than the 20+ seconds it can take to compile complex procedural macros and their dependencies.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: write
</div>