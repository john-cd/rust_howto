# Rust scripting

{{#include rhai.incl.md}}

## Embed Rust scripting in your application {#rust-scripting}

[![rhai][c-rhai-badge]][c-rhai]{{hi:rhai}}
[![rhai-crates.io][c-rhai-crates.io-badge]][c-rhai-crates.io]
[![rhai-github][c-rhai-github-badge]][c-rhai-github]
[![rhai-lib.rs][c-rhai-lib.rs-badge]][c-rhai-lib.rs]
[![cat-embedded][cat-embedded-badge]][cat-embedded]{{hi:Embedded development}}
[![cat-parser-implementations][cat-parser-implementations-badge]][cat-parser-implementations]{{hi:Parser implementations}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}
[![cat-wasm][cat-wasm-badge]][cat-wasm]{{hi:WebAssembly}}

[rhai][c-rhai-website]{{hi:rhai}}⮳ is an embedded scripting language and evaluation engine for Rust that gives a safe and easy way to add scripting to any application.{{hi:Scripting in Rust}}

Features:

- Similar to JavaScript+Rust with dynamic typing.
- Tight integration with native Rust [functions][p-functions] and types.
- Sand-boxing - the [scripting][p-scripting] engine, if declared immutable, cannot mutate the containing environment unless explicitly permitted.

```rust,editable
{{#include ../../../crates/other/tests/scripting/rhai.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[rhai: write](https://github.com/john-cd/rust_howto/issues/610)
</div>
