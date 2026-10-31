<!-- TODO Reviewed ??-??-2026 -->

# Interfacing with JavaScript

{{#include interfacing_with_javascript.incl.md}}

WebAssembly modules do not have JavaScript or browser access by themselves. On a JavaScript host, [`wasm-bindgen`][c~wasm-bindgen~docs]↗{{hi:wasm-bindgen}} generates bindings so JavaScript and Rust can call across the module boundary and exchange supported values. Use these bindings with a JavaScript-compatible target and runtime.

## Export Rust functions to JavaScript {#wasm-bindgen}

Annotate a Rust function with `#[wasm_bindgen]` to export it to JavaScript. The example exports `add` only for the browser target while keeping the arithmetic function available to native tests:

```rust,editable
{{#include ../../../crates/cats/wasm/examples/wasm_development/wasm.rs:example}}
```

JavaScript imports the generated module and awaits initialization before calling an export. For a package named `my_app` built with the `web` target, the generated module can be used like this:

```javascript
import init, { add } from "./pkg/my_app.js";

await init();
console.log(add(2, 3));
```

See the [wasm-bindgen guide][book~wasm-bindgen]↗ for the binding model and supported conversions.

## Import JavaScript functions into Rust {#js-import}

An `extern "C"` block annotated with `#[wasm_bindgen]` declares JavaScript functions that Rust can call. Rust 2024 requires foreign blocks to be `unsafe`; the imported call is therefore made in an explicit unsafe block. This example imports `console.log`:

```rust,editable
{{#include ../../../crates/cats/wasm/examples/interfacing_with_javascript/js_import.rs:example}}
```

## Use JavaScript built-ins with `js-sys` {#js-sys}

[![js-sys~website][c~js-sys~website~badge]][c~js-sys~website] {{#badges js-sys}} {{#categories wasm=WebAssembly}}

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

## Convert structured values with Serde {#serde-wasm-bindgen}

[`serde-wasm-bindgen`][c~serde-wasm-bindgen~docs]↗ converts between `JsValue` and Serde data types. It represents values as JavaScript objects, arrays, and primitives rather than encoding everything as JSON. Add it as a target-specific dependency when only the WebAssembly build needs the conversion.

This example accepts a JavaScript object, deserializes it, increments a field, and returns the updated object:

```rust,editable
{{#include ../../../crates/cats/wasm/examples/interfacing_with_javascript/serde_wasm_bindgen.rs:example}}
```

## Await JavaScript Promises {#wasm-bindgen-futures}

[`wasm-bindgen-futures`][c~wasm-bindgen-futures~docs] provides `JsFuture` to await a JavaScript `Promise` from Rust. An exported `async` function is exposed to JavaScript as a `Promise`; propagate rejection with `?` and validate the resolved value before using it.

```rust,editable
{{#include ../../../crates/cats/wasm/examples/interfacing_with_javascript/promises.rs:example}}
```

## Build and test

For a browser library crate, install the target and `wasm-pack`, then build the JavaScript package for the browser's native ES module loader:

```sh
rustup target add wasm32-unknown-unknown
cargo install --locked wasm-pack
wasm-pack build --target web
```

The resulting package includes generated JavaScript glue and a WebAssembly module. Import its generated JavaScript module, call its async initializer, and then use the exported functions. This repository's examples crate is not a library package for `wasm-pack`; build its examples with Cargo and the `wasm32-unknown-unknown` target instead.

Keep pure logic separate from JavaScript calls so native Cargo tests can exercise it. For behavior that needs JavaScript or browser APIs, add `wasm-bindgen-test` and run the tests in Node.js or a browser, for example with `wasm-pack test --node` or `wasm-pack test --headless --chrome`. See the [browser test guide][wasm-bindgen-test~guide]↗.

## Related topics {#related-topics .skip}

- [[wasm_basics | WASM Basics]]
- [[wasm_development | WASM Development]]
- [[programming_languages | Programming Languages]]
- [[web-programming | Web Programming]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
