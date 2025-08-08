# WebAssembly (WASM)

[![cat~wasm][cat~wasm~badge]][cat~wasm]

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

## Build an Entire Web Application in Rust Using WASM-based Web Frameworks

The [`yew`][c~yew~docs]↗{{hi:yew}}, [`seed`][c~seed~docs]↗{{hi:seed}}, [`leptos`][c~leptos~docs]↗{{hi:leptos}} frameworks enable building complex web applications with Rust/WASM.

{{#include yew.incl.md}}

{{#include leptos.incl.md}}

## See Also

[Rust and WebAssembly (book)][book~rustwasm]{{hi:WASM}}↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/970)
Browser-based Execution.
Node.js with WASM.
Cover [`wat`][c~wat~docs]↗{{hi:wat}}, [`parity-wasm`][c~parity-wasm~docs]↗{{hi:parity-wasm}}

- [WASIX - The Superset of WASI](https://wasix.org/)
- [wasmtime: A fast and secure runtime for WebAssembly](https://github.com/bytecodealliance/wasmtime)
- [WASI.dev](https://wasi.dev/)

- [Sycamore](https://sycamore.dev/)
- [sycamore: A library for creating reactive web apps in Rust and WebAssembly](https://github.com/sycamore-rs/sycamore)

</div>
