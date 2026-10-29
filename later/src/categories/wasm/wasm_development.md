<!-- TODO Reviewed ??-??-2026 -->

# WASM Development

{{#include wasm_runtimes.incl.md}}

## Choose a workflow {#choose-a-workflow .skip}

Choose the target to match the host:

| Target                   | Use                                                 | Host requirements                                                                                  |
| ------------------------ | --------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| `wasm32-unknown-unknown` | Browser and JavaScript-hosted modules.              | Provides no operating-system APIs; use JavaScript bindings for host services.                      |
| `wasm32-wasip1`          | Programs using the WASI Preview 1 system interface. | A runtime that supports Preview 1; useful when maintaining compatibility with existing WASI hosts. |
| `wasm32-wasip2`          | Programs using the WASI Preview 2 component model.  | A runtime that supports Preview 2 components.                                                      |

A browser application usually combines `wasm32-unknown-unknown` with a UI framework such as [[yew | Yew]] or [[leptos | Leptos]] and Trunk. A Rust library called from an existing JavaScript application can use `wasm-bindgen` and `wasm-pack`. For a command-line program or server module, choose a WASI target supported by the deployment runtime. The WASI Preview 1 target produces a core Wasm module; Preview 2 uses the component model.

Install the target and build for it explicitly:

```bash
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```

For WASI, replace the target name in both commands with `wasm32-wasip1` or `wasm32-wasip2`. The Preview 2 target requires a component-capable runtime. [`cargo-component`][cargo-component~repo]↗ can build Rust crates as WebAssembly components; its maintainers currently describe it as experimental, so pin the tool version and check its support notes before adopting it.

```bash
cargo install --locked cargo-component
cargo component build --release
```

## Compile a Rust library for JavaScript

Install the browser target, create a library crate, and configure it to produce a dynamic library for WebAssembly:

```bash
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

```bash
cargo install --locked wasm-pack
wasm-pack build --target web
```

The generated module can be imported by browser JavaScript. Initialize it before calling its exports:

```javascript
import init, { add } from "./pkg/hello_wasm.js";

await init();
console.log(add(2, 2));
```

Use `--target bundler` when a JavaScript bundler owns the application build. Keep the generated JavaScript glue and Wasm package in sync with the Rust crate.

## Build a browser application

For a framework-based front end, use the tool supported by that framework. For example, serve a Yew or Leptos client-side application locally with `trunk serve --open` and build it for deployment with `trunk build --release`. Trunk reads the application entry point from `index.html` and builds its declared Rust and web assets. The browser still accesses DOM and other Web Platform APIs through JavaScript bindings; WebAssembly does not provide direct DOM access.

## Test Wasm and JavaScript integration

Keep logic that does not call host APIs testable on the host with `cargo test`. Use [`wasm-bindgen-test`][c~wasm-bindgen-test~docs]↗{{hi:wasm-bindgen-test}} ([guide][wasm-bindgen-test~guide]↗) for behavior that needs JavaScript imports, browser globals, or DOM APIs. Mark those tests with `#[wasm_bindgen_test]` and choose the JavaScript runtime or browser that matches the feature under test:

```bash
wasm-pack test --node
wasm-pack test --headless --chrome
```

Node.js is useful for tests that need JavaScript but not a browser DOM. Use a headless browser to exercise browser APIs. These runs require the selected runtime and, for browser tests, a supported browser installation.

## Improve execution speed and binary size

Start with an optimized Cargo release build and measure the result. Cargo's release profile can prioritize size and enable link-time optimization:

```toml
[profile.release]
opt-level = "z" # Optimize for size and disable loop vectorization.
lto = "thin"
```

`opt-level = "s"` also optimizes for size but keeps loop vectorization enabled. The smaller setting is not guaranteed to produce a smaller or faster program; compare release artifacts and run them in the target host. Link-time optimization can improve whole-program optimization at the cost of longer build times. See the [Cargo profile reference][rust~cargo-profiles~book]↗.

[Binaryen][binaryen~repo]↗ provides `wasm-opt`, a post-link optimizer. For a standalone module, a size-focused pass can be run as follows:

```bash
wasm-opt input.wasm -Oz -o output.wasm
```

When a tool such as Trunk or `wasm-pack` produces JavaScript glue alongside the module, keep the artifacts paired and test the optimized package through the same deployment path. [Twiggy][twiggy~repo]↗ can show which functions and data contribute to a module's size:

```bash
twiggy top path/to/module.wasm
```

Twiggy's upstream repository was archived in March 2026. Treat it as a tool for inspecting existing workflows, and check its maintenance status before adding it to a new toolchain.

## Use SIMD in Wasm

WebAssembly SIMD provides 128-bit vector operations through `core::arch::wasm32`. To enable SIMD throughout a build, pass the target feature to rustc:

```bash
RUSTFLAGS="-C target-feature=+simd128" cargo build --release --target wasm32-unknown-unknown
```

Code can also use architecture-specific intrinsics in `core::arch::wasm32` and gate specialized functions with `#[target_feature(enable = "simd128")]`. A Wasm binary that contains SIMD instructions needs a runtime that supports the SIMD proposal. WebAssembly does not provide the same runtime CPU feature detection used by native Rust targets, so publish a compatible non-SIMD build when the host matrix requires one. See the [Rust Wasm SIMD documentation][rustc~wasm-simd~docs]↗ and the [`core::arch::wasm32` API][rust~core-arch-wasm32~docs]↗.

## Profile a Rust-Wasm application

Profile the release artifact in its actual host. Browser developer tools can record CPU activity, network requests, startup, and user interactions; use the Performance panel to find work that affects page responsiveness. Measure both Wasm execution and time spent in JavaScript, browser APIs, downloads, and initialization.

During development, [`console_error_panic_hook`][c~console_error_panic_hook~docs]↗ can forward Rust panic messages to `console.error` so the browser console can show useful diagnostics. For structured events and timing in the browser console, [`tracing-wasm`][tracing-wasm~docs]↗ provides a tracing layer that uses browser performance and console APIs. Enable diagnostic logging in development builds and compare a release build before drawing performance conclusions.

## Related topics {#related-topics .skip}

- [[wasm_basics | WASM Basics]]
- [[interfacing_with_javascript | Interfacing with JavaScript]]
- [[yew | Yew]]
- [[leptos | Leptos]]
- [[wasm_standalone_runtimes | WASM Standalone Runtimes]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
