# Interfacing with JavaScript (`wasm-bindgen` and `web-sys`)

{{#include interfacing_with_javascript.incl.md}}

## Calling external JavaScript functions from Rust with `wasm-bindgen` {#wasm-bindgen}

[`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}} handles the complexities of passing data between Rust/WASM and JavaScript.

[`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}} is essential for bridging between Rust and JavaScript, allowing you to interact with the browser's APIs from your Rust/WASM code.

The [`wasm-bindgen`](https://rustwasm.github.io/docs/wasm-bindgen) guide covers the `wasm-bindgen` tool and crate. [`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}} is a Rust library and CLI tool that facilitate high-level interactions between WASM modules and JavaScript.

## Bind to JS global objects and functions found in all JS environments with `js-sys` {#js-sys}

[![js-sys-website][c-js_sys-website-badge]][c-js_sys-website] [![js-sys][c-js_sys-badge]][c-js_sys] [![js-sys-crates.io][c-js_sys-crates.io-badge]][c-js_sys-crates.io] [![js-sys-github][c-js_sys-github-badge]][c-js_sys-github] [![js-sys-lib.rs][c-js_sys-lib.rs-badge]][c-js_sys-lib.rs]{{hi:js-sys}} [![cat-wasm][cat-wasm-badge]][cat-wasm]{{hi:WebAssembly}}

Bindings for all JS global objects and functions found in all JS environments like Node.js and browsers, built on `#[wasm_bindgen]` using the [`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}} crate.

This does not include any Web, Node, or any other JS environment APIs. Only the things that are guaranteed to exist in the global scope by the ECMAScript standard.

## Accessing DOM and Web APIs via Rust {#skip}

The `web-sys` crate provides Rust bindings to the Web's APIs, allowing you to interact with the DOM, Canvas, WebGL, and other browser features.
Essentially, `web-sys` is the bridge that lets your Rust code talk to the browser.

## Calling Rust/WASM functions from JavaScript {#skip1}

[`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}} also facilitates calling Rust/WASM functions from JavaScript.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write
cover `web-sys`
</div>
