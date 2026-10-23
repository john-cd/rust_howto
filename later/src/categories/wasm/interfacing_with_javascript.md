<!-- TODO Reviewed ??-??-2026 -->
# Interfacing with JavaScript

{{#include interfacing_with_javascript.incl.md}}

WebAssembly modules do not have JavaScript or browser access by themselves. On a JavaScript host, [`wasm-bindgen`][c~wasm-bindgen~docs]↗{{hi:wasm-bindgen}} generates bindings so JavaScript and Rust can call across the module boundary and exchange supported values. Use it only with a JavaScript-compatible target and runtime.

## Import and export functions with `wasm-bindgen` {#wasm-bindgen}

Mark a Rust function with `#[wasm_bindgen]` to export it to JavaScript. An `extern "C"` declaration annotated with `#[wasm_bindgen]` can import a JavaScript function into Rust. The generated JavaScript glue initializes the module and handles the values supported by the binding.

See the [wasm-bindgen guide][book~wasm-bindgen]↗ for the binding model and the [[wasm_development | WASM Development]] chapter for a complete build workflow.

## Use JavaScript built-ins with `js-sys` {#js-sys}

[![js-sys~website][c~js-sys~website~badge]][c~js-sys~website] [![js-sys][c~js-sys~docs~badge]][c~js-sys~docs] [![js-sys~crates.io][c~js-sys~crates.io~badge]][c~js-sys~crates.io] [![js-sys~repo][c~js-sys~repo~badge]][c~js-sys~repo] [![js-sys~lib.rs][c~js-sys~lib.rs~badge]][c~js-sys~lib.rs]{{hi:js-sys}} [![cat~wasm][cat~wasm~badge]][cat~wasm]{{hi:WebAssembly}}

[`js-sys`][c~js-sys~docs]↗{{hi:js-sys}} provides bindings to standard ECMAScript objects and functions, such as `Date`, `Array`, and `Promise`. It does not include browser DOM or Node.js APIs. The example exports a Rust function that reads the current time through JavaScript's `Date` object.

```rust,editable
{{#include ../../../crates/cats/wasm/examples/interfacing_with_javascript/js_sys.rs:example}}
```

## Use browser APIs with `web-sys` {#web-sys}

[`web-sys`][c~web-sys~docs]↗{{hi:web-sys}} provides bindings to Web Platform APIs such as `Window`, `Document`, and the DOM. Enable the Cargo features for the API types and methods the application uses. These APIs are available only when the host provides them; a Node.js process does not provide a browser DOM.

The example obtains the browser window and reports its absence or an API error instead of assuming that a window is always available:

```rust,editable
{{#include ../../../crates/cats/wasm/examples/interfacing_with_javascript/web_sys.rs:example}}
```

For component-based interfaces, use the framework's view and event model where it fits; direct `web-sys` calls are useful for browser features the framework does not wrap.

## Call Rust from JavaScript {#calling-rust-wasm .skip}

The generated JavaScript module exports the functions annotated with `#[wasm_bindgen]`. With the `web` target, import the module, await its initialization, and then call the exports. Keep the package generated from the Rust crate in sync with the JavaScript application that imports it.

JavaScript and Rust have different type and memory models. Keep boundary conversions explicit, handle JavaScript exceptions or unavailable APIs, and avoid crossing the boundary for every item in a large data set when a coarser call will do.

## Related topics {#related-topics .skip}

- [[wasm_basics | WASM Basics]]
- [[wasm_development | WASM Development]]
- [[programming_languages | Programming Languages]]
- [[web-programming | Web Programming]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1218)
</div>
