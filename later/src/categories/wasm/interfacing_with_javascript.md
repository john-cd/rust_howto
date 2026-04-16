# Interfacing with JavaScript (`wasm-bindgen` and `web-sys`)

{{#include interfacing_with_javascript.incl.md}}

## Calling External JavaScript Functions from Rust with `wasm-bindgen` {#wasm-bindgen}

[`wasm-bindgen`][c~wasm-bindgen~docs]↗{{hi:wasm-bindgen}} handles the complexities of passing data between Rust/WASM and JavaScript.

[`wasm-bindgen`][c~wasm-bindgen~docs]↗{{hi:wasm-bindgen}} is essential for bridging between Rust and JavaScript, allowing you to interact with the browser's APIs from your Rust/WASM code.

The [`wasm-bindgen`][book~wasm-bindgen]↗ guide covers the `wasm-bindgen` tool and crate. [`wasm-bindgen`][c~wasm-bindgen~docs]↗{{hi:wasm-bindgen}} is a Rust library and CLI tool that facilitate high-level interactions between WASM modules and JavaScript.

## Bind to JS Global Objects and Functions Found in All JS Environments with `js-sys` {#js-sys}

[![js-sys~website][c~js-sys~website~badge]][c~js-sys~website] [![js-sys][c~js-sys~docs~badge]][c~js-sys~docs] [![js-sys~crates.io][c~js-sys~crates.io~badge]][c~js-sys~crates.io] [![js-sys~repo][c~js-sys~repo~badge]][c~js-sys~repo] [![js-sys~lib.rs][c~js-sys~lib.rs~badge]][c~js-sys~lib.rs]{{hi:js-sys}} [![cat~wasm][cat~wasm~badge]][cat~wasm]{{hi:WebAssembly}}

Bindings for all JS global objects and functions found in all JS environments like Node.js and browsers, built on `#[wasm_bindgen]` using the [`wasm-bindgen`][c~wasm-bindgen~docs]↗{{hi:wasm-bindgen}} crate.

This does not include any Web, [Node][p~node], or any other JS environment APIs. Only the things that are guaranteed to exist in the global scope by the ECMAScript standard.

TODO add example ?

## Accessing DOM and Web APIs via Rust {#accessing-dom .skip}

The [`web-sys`][c~web-sys~crates.io]↗{{hi:web-sys}} crate provides Rust bindings to the Web's APIs, allowing you to interact with the DOM, Canvas, WebGL, and other browser features.
Essentially, [`web-sys`][c~web-sys~docs]↗{{hi:web-sys}} is the bridge that lets your Rust code talk to the browser.

TODO add example ?

## Calling Rust/WASM Functions from JavaScript {#calling-rust-wasm .skip}

[`wasm-bindgen`][c~wasm-bindgen~docs]↗{{hi:wasm-bindgen}} also facilitates calling Rust/WASM functions from JavaScript.

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1218)
</div>
