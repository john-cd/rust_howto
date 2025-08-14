# Rust Scripting

{{#include rhai.incl.md}}

## Embed Rust Scripting in Your Application {#rust-scripting}

[![rhai][c~rhai~docs~badge]][c~rhai~docs]{{hi:rhai}}
[![rhai~crates.io][c~rhai~crates.io~badge]][c~rhai~crates.io]
[![rhai~github][c~rhai~github~badge]][c~rhai~github]
[![rhai~lib.rs][c~rhai~lib.rs~badge]][c~rhai~lib.rs]
[![cat~embedded][cat~embedded~badge]][cat~embedded]{{hi:Embedded development}}
[![cat~parser-implementations][cat~parser-implementations~badge]][cat~parser-implementations]{{hi:Parser implementations}}
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}
[![cat~wasm][cat~wasm~badge]][cat~wasm]{{hi:WebAssembly}}

[rhai][c~rhai~website]↗{{hi:rhai}} is an embedded scripting language and evaluation engine that gives a safe and easy way to add scripting to any Rust application.{{hi:Scripting in Rust}}

Features:

- Similar to JavaScript + Rust, with dynamic typing.
- Tight integration with native Rust [functions][p~functions] and types.
- Function Registration: it can expose Rust functions into scripts.
- Sand-boxing: the [scripting][p~scripting] engine, if declared immutable, cannot mutate the containing environment unless explicitly permitted.
- Robust error reporting.

```rust,editable
{{#include ../../../crates/other/examples/scripting/rhai.rs:example}}
```

## Rhai Ecosystem {#skip}

There are independent packages that provides additional functionality, for example:

- [`rhai-fs`][c~rhai-fs~docs]↗{{hi:rhai-fs}} provides file system access within Rhai scripts.
- [`rhai-rand`][c~rhai-rand~docs]↗{{hi:rhai-rand}} provides random number generation using the [`rand`][c~rand~docs]↗{{hi:rand}} crate and array shuffling and sampling.
- [`rhai-sci`]c~rhai-sci~docs↗{{hi:rhai-sci}} provides functions useful for scientific computing, inspired by languages like MATLAB, Octave, and R.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[rhai: write](https://github.com/john-cd/rust_howto/issues/610)
</div>
