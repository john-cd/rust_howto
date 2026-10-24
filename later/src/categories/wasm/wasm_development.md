<!-- TODO Reviewed ??-??-2026 -->

# WASM Development

{{#include wasm_runtimes.incl.md}}

## Choose a workflow {#choose-a-workflow .skip}

For a browser application, use a WebAssembly-enabled UI framework such as [[yew | Yew]] or [[leptos | Leptos]] with a web application tool such as Trunk. These tools build and serve the HTML, JavaScript bindings, and WebAssembly module together.

For a Rust library that an existing JavaScript application will call, use [`wasm-bindgen`][c~wasm-bindgen~docs]↗{{hi:wasm-bindgen}} with [`wasm-pack`][c~wasm-pack~crates.io]↗{{hi:wasm-pack}}. For a command-line program or server module, select a WASI target and runtime as described above.

## Build a Rust library for JavaScript

Install the browser target, create a library crate, and configure it to produce a dynamic library for WebAssembly:

```sh
rustup target add wasm32-unknown-unknown
cargo new --lib hello_wasm
cd hello_wasm
cargo add wasm-bindgen
```

Add the library crate types to `Cargo.toml`:

```toml
[lib]
crate-type = ["cdylib", "rlib"]
```

Export a Rust function with `wasm-bindgen`:

```rust,editable
{{#include ../../../crates/cats/wasm/examples/wasm_development/wasm.rs:example}}
```

Build JavaScript bindings and a browser-loadable ES module with `wasm-pack`:

```sh
cargo install --locked wasm-pack
wasm-pack build --target web
```

The generated module can be imported by browser JavaScript. Initialization loads the Wasm module before calling its exports:

```javascript
import init, { add } from "./pkg/hello_wasm.js";

await init();
console.log(add(2, 2));
```

Use `--target bundler` for a package intended for a JavaScript bundler, or another supported target when the host is Node.js or a different JavaScript runtime. The generated JavaScript glue code is part of the interface; keep the Rust and JavaScript packages in sync when it changes.

## Build a browser application

For a framework-based front end, install the `wasm32-unknown-unknown` target and use the tool supported by that framework. For example, a Yew or Leptos client-side rendering project can be served locally with `trunk serve --open` and built for deployment with `trunk build --release`. The application still calls browser APIs through JavaScript bindings; WebAssembly does not provide direct DOM access.

## Test and debug

Keep business logic that does not need browser APIs testable on the host with `cargo test`. To exercise JavaScript imports and browser-facing code, use [`wasm-bindgen-test`][c~wasm-bindgen-test~docs]↗{{hi:wasm-bindgen-test}} with `wasm-pack test --node`, or configure a browser runner such as `wasm-pack test --chrome --headless`. These runs require the corresponding JavaScript runtime or browser tooling.

Use browser developer tools to inspect JavaScript and WebAssembly execution. [`console_error_panic_hook`][c~console_error_panic_hook~docs]↗{{hi:console_error_panic_hook}} can route Rust panic messages to the browser console during development. Test the release artifact in its actual host because its imports, initialization, and deployment headers must all work there.

## Measure and optimize

Build and measure the production artifact before optimizing. Inspect module size with [`twiggy`][c~twiggy~docs]↗{{hi:twiggy}}; [`wasm-opt`][c~wasm-opt~docs]↗{{hi:wasm-opt}} can perform additional optimizations when it fits the toolchain. Profile the application in its target runtime, and account for module download and initialization, memory use, and data copied across the JavaScript boundary. Choose an allocator or size optimization only after measurements show it helps.

## Related topics {#related-topics .skip}

- [[wasm_basics | WASM Basics]]
- [[interfacing_with_javascript | Interfacing with JavaScript]]
- [[yew | Yew]]
- [[leptos | Leptos]]
- [[wasm_standalone_runtimes | WASM Standalone Runtimes]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1217)
</div>
