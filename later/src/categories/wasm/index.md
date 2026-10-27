<!-- TODO Reviewed ??-??-2026 -->

# WebAssembly (WASM)

[![cat~wasm][cat~wasm~badge]][cat~wasm]

Tooling targeting WebAssembly or manipulating WebAssembly.

WebAssembly (Wasm) is a portable binary instruction format. Rust compiles to it for the browser (with JavaScript glue) and for server-side runtimes that use the WebAssembly System Interface (WASI).

## Pages

- [[wasm_basics | WASM Basics]]: concepts and use cases.
- [[wasm_development | WASM Development]]: toolchain, `wasm-bindgen`, `wasm-pack`, and running Wasm in the browser or with Node.js.
- [[interfacing_with_javascript | Interfacing with JavaScript]]: calling between Rust and JavaScript.
- [[leptos | Leptos]] and [[yew | Yew]]: front-end frameworks.
- [[wasm_standalone_runtimes | WASM Standalone Runtimes]]: running Wasm outside the browser.

## Run Wasm Outside the Browser

[`wasmtime`][c~wasmtime~repo]↗{{hi:wasmtime}} and [`wasmer`][c~wasmer~repo]↗{{hi:wasmer}} embed Wasm in a Rust program or run it as a standalone runtime. [WASI][wasi~website]↗{{hi:WASI}} gives modules a portable system interface; [WASIX][wasix~website]↗{{hi:WASIX}} is Wasmer's extension adding threads and sockets.

{{#include wasm_runtimes.incl.md}}

## Reading and Writing Wasm

Use [`wat`][c~wat~docs]↗{{hi:wat}} to convert text format to binary, and [`wasm-encoder`][c~wasm-encoder~docs]↗{{hi:wasm-encoder}} and [`wasmparser`][c~wasmparser~docs]↗{{hi:wasmparser}} (part of `wasm-tools`) to build and parse modules. [`parity-wasm`][c~parity-wasm~docs]↗{{hi:parity-wasm}} is deprecated.

## Rust Front-End Frameworks

[`yew`][c~yew~docs]↗{{hi:yew}}, [`leptos`][c~leptos~docs]↗{{hi:leptos}}, [`dioxus`][c~dioxus~docs]↗{{hi:dioxus}}, [`sycamore`][c~sycamore~repo]↗{{hi:sycamore}} and [`seed`][c~seed~docs]↗{{hi:seed}} build entire web applications in Rust. See also [[web-programming | Web Programming]]. Dioxus's web renderer targets WebAssembly in the browser; its [desktop and mobile webview options](../gui/web_based_gui.md#dioxus) are covered in the GUI chapter.

{{#include interfacing_with_javascript.incl.md}}

{{#include yew.incl.md}}

{{#include leptos.incl.md}}

## References

- [Rust and WebAssembly (book)][book~rustwasm]↗{{hi:WASM}}.

## Related Topics

- [[wasm_basics | WASM Basics]].
- [[wasm_development | WASM Development]].
- [[interfacing_with_javascript | Interfacing with Javascript]].
- [[web-programming | Web Programming]].
- [[virtualization | Virtualization]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
