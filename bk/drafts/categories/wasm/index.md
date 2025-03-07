# WebAssembly (WASM)

[![cat-wasm][cat-wasm-badge]][cat-wasm]

Tooling targeting WebAssembly or manipulating WebAssembly.

## WASM Basics and Use Cases

See [[wasm_basics | WASM Basics]].

## WASM Development Process

See [[wasm_development | WASM Development]].

## WASM Runtime Environments

- Browser-based Execution.
- Node.js with WASM.
- [[wasm_standalone_runtimes | WASM Standalone Runtimes]].
- Embedded Systems with WASM in Rust.

{{#include wasm_runtimes.incl.md}}

## Interfacing Rust WASM and JavaScript

See [[interfacing_with_javascript | Interfacing with Javascript]].

{{#include interfacing_with_javascript.incl.md}}

## Build an entire Web application in Rust using WASM-based Web Frameworks

The [`yew`][c-yew]⮳{{hi:yew}}, [`seed`][c-seed]⮳{{hi:seed}}, [`leptos`][c-leptos]⮳{{hi:leptos}} frameworks enable building complex web applications with Rust/WASM.

{{#include yew.incl.md}}

{{#include leptos.incl.md}}

## See also

[Rust and WebAssembly (book)][book-rustwasm]{{hi:WASM}}⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/970)
Browser-based Execution
Node.js with WASM
</div>
