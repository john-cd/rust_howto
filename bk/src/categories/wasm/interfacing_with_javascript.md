<!-- TODO Reviewed ??-??-2026. -->

# Interfacing with JavaScript

{{#include interfacing_with_javascript.incl.md}}

WebAssembly modules do not have JavaScript or browser access by themselves. On a JavaScript host, [`wasm-bindgen`][c~wasm-bindgen~docs]↗{{hi:wasm-bindgen}} generates bindings so JavaScript and Rust can call across the module boundary and exchange supported values. Use these bindings with a JavaScript-compatible target and runtime.

## Export Rust functions to JavaScript {#wasm-bindgen}

Annotate a Rust function with `#[wasm_bindgen]` to export it to JavaScript. The example exports `add` only for the browser target while keeping the arithmetic function available to native tests:

```rust,noplayground
{{#include ../../../platform/cats/wasm/examples/wasm_development/wasm.rs:example}}
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

```rust,noplayground
{{#include ../../../platform/cats/wasm/examples/interfacing_with_javascript/js_import.rs:example}}
```

## Use JavaScript built-ins with `js-sys` {#js-sys}

[![js-sys~website][c~js-sys~website~badge]][c~js-sys~website] {{#badges js-sys}} {{#categories wasm=WebAssembly}}

[`js-sys`][c~js-sys~docs]↗{{hi:js-sys}} provides bindings to standard ECMAScript objects and functions, such as `Date`, `Array`, and `Promise`. It does not include browser DOM or Node.js APIs. The example exports a Rust function that reads the current time through JavaScript's `Date` object.

```rust,noplayground
{{#include ../../../platform/cats/wasm/examples/interfacing_with_javascript/js_sys.rs:example}}
```

## Use browser APIs with `web-sys` {#web-sys}

[`web-sys`][c~web-sys~docs]↗{{hi:web-sys}} provides bindings to Web Platform APIs such as `Window`, `Document`, and the DOM. Enable the Cargo features for the API types and methods the application uses. These APIs are available only when the host provides them; a Node.js process does not provide a browser DOM.

The example obtains the browser window and reports its absence or an API error instead of assuming that a window is always available:

```rust,noplayground
{{#include ../../../platform/cats/wasm/examples/interfacing_with_javascript/web_sys.rs:example}}
```

For component-based interfaces, use the framework's view and event model where it fits; direct `web-sys` calls are useful for browser features the framework does not wrap.

## Convert structured values with Serde {#serde-wasm-bindgen}

[`serde-wasm-bindgen`][c~serde-wasm-bindgen~docs]↗ converts between `JsValue` and Serde data types. It represents values as JavaScript objects, arrays, and primitives rather than encoding everything as JSON. Add it as a target-specific dependency when only the WebAssembly build needs the conversion.

This example accepts a JavaScript object, deserializes it, increments a field, and returns the updated object:

```rust,noplayground
{{#include ../../../platform/cats/wasm/examples/interfacing_with_javascript/serde_wasm_bindgen.rs:example}}
```

## Await JavaScript Promises {#wasm-bindgen-futures}

[`wasm-bindgen-futures`][c~wasm-bindgen-futures~docs] provides `JsFuture` to await a JavaScript `Promise` from Rust. An exported `async` function is exposed to JavaScript as a `Promise`; propagate rejection with `?` and validate the resolved value before using it.

```rust,noplayground
{{#include ../../../platform/cats/wasm/examples/interfacing_with_javascript/promises.rs:example}}
```

## Run a JavaScript Round Trip in Node.js {#node-round-trip}

This standalone crate exports a greeting as a JavaScript function. The generated glue converts a JavaScript string to Rust UTF-8 and turns `Err(JsError)` into a JavaScript exception.

```rust,noplayground
{{#include ../../../platform/cats/wasm/js_round_trip/src/lib.rs:example}}
```

The Node script checks both paths. The Unicode greeting exercises string conversion on both sides of the boundary.

```javascript
{{#include ../../../platform/cats/wasm/js_round_trip/smoke.cjs:example}}
```

Install Node.js and check that `wasm-bindgen --version` reports `0.2.130`,
the version pinned in the guest's manifest. If you need that CLI version,
run `cargo install --locked wasm-bindgen-cli --version 0.2.130`. From the
repository root, run:

```sh
rustup target add wasm32-unknown-unknown
cargo build --locked --release --target wasm32-unknown-unknown \
  --target-dir target/wasm-js-demo --manifest-path bk/platform/cats/wasm/js_round_trip/Cargo.toml
wasm-bindgen --target nodejs --out-dir target/wasm-js-demo/node \
  target/wasm-js-demo/wasm32-unknown-unknown/release/wasm_js_round_trip.wasm
node bk/platform/cats/wasm/js_round_trip/smoke.cjs \
  target/wasm-js-demo/node/wasm_js_round_trip.js
```

The test prints `Wasm greeting and JavaScript error checked.` after both assertions.

## Run the Same Guest in a Browser {#browser-round-trip}

Generate browser bindings from the same `.wasm` file. The smoke page imports
the generated module and awaits its initializer before calling `greet`.
Copy the page beside the generated files and serve that directory over HTTP
with Python 3:

```sh
wasm-bindgen --target web --out-dir target/wasm-js-demo/web \
  target/wasm-js-demo/wasm32-unknown-unknown/release/wasm_js_round_trip.wasm
cp bk/platform/cats/wasm/js_round_trip/smoke.html target/wasm-js-demo/web/index.html
python -m http.server 8000 --bind 127.0.0.1 --directory target/wasm-js-demo/web
```

Open `http://127.0.0.1:8000` in a browser. The page displays
`Hello, Zoë 🌱! Empty-name error checked.` when both calls behave as expected.
For Rust test suites that exercise DOM interactions, see
[`wasm-bindgen-test`][wasm-bindgen-test~guide]↗.

## Related topics {#related-topics .skip}

- [[wasm_basics | WASM Basics]]
- [[wasm_development | WASM Development]]
- [[programming_languages | Programming Languages]]
- [[web-programming | Web Programming]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
