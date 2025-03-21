# WASM Development

{{#include wasm_runtimes.incl.md}}

A typical Rust/WASM development workflow involves writing Rust code, using [`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}} to create bindings to JavaScript, compiling the Rust code to WASM, and then using JavaScript to load and interact with the WASM module in a web page.

## Compiling Rust to WASM (wasm32-unknown-unknown Compilation target) {#skip}

## Tools and Frameworks (e.g., `wasm-pack`, `cargo-generate`) {#skip1}

### `wasm-pack` {#wasm-pack}

`wasm-pack` helps compile the code to WebAssembly and produce the right packaging for use in the browser.

The [`wasm-pack`](https://rustwasm.github.io/docs/wasm-pack) book covers the Rust to WebAssembly workflow tool. This book covers prerequistes, project setup, and available commands for `wasm-pack`.

```sh
# Download and install
cargo install wasm-pack
# Create a new library project
cargo new --lib hello-wasm
```

Enter the following Rust code in `src/lib.rs`

```rust,editable
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
```

`wasm-pack` uses [`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}} to provide a bridge between the types of JavaScript and Rust. It allows JavaScript to call a Rust API with a string, or a Rust function to catch a JavaScript exception.

### Memory Management in WASM (e.g., `std::alloc`) {#skip2}

[`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}} helps manage WASM memory. [`wee_alloc`][c-wee_alloc]⮳{{hi:wee_alloc}} is a memory allocator designed for WASM.

### Testing and Debugging {#skip3}

#### Unit Testing WASM Modules {#skip4}

[`wasm-bindgen-test`][c-wasm_bindgen_test]⮳{{hi:wasm-bindgen-test}} provides utilities for testing your Rust/WASM code.

#### Integration Testing with JavaScript {#skip5}

#### Debugging WASM {#skip6}

Browser developer tools can be used to debug WASM.

[`console_error_panic_hook`][c-console_error_panic_hook]⮳{{hi:console_error_panic_hook}} helps with better error reporting in the browser console. |

### Performance Optimization {#skip7}

[`wee_alloc`][c-wee_alloc]⮳{{hi:wee_alloc}} can help reduce WASM binary size. [`twiggy`][c-twiggy]⮳{{hi:twiggy}} is a tool for analyzing WASM binaries. [`wasm-opt`][c-wasm_opt]⮳{{hi:wasm-opt}} is a tool for optimizing WASM code.

#### Reducing WASM Binary Size {#skip8}

WASM binary size is important for web performance. Use tools like [`wee_alloc`][c-wee_alloc]⮳{{hi:wee_alloc}}, [`twiggy`][c-twiggy]⮳{{hi:twiggy}}, and [`wasm-opt`][c-wasm_opt]⮳{{hi:wasm-opt}} to reduce size.

#### Improving Execution Speed {#skip9}

#### Leveraging SIMD in WASM {#skip}

#### Profiling Rust-WASM Applications {#skip}

## References {#skip}

[Compiling from Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm)⮳.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1217)
[write](https://github.com/john-cd/rust_howto/issues/1217) Improving Execution Speed
[write](https://github.com/john-cd/rust_howto/issues/1217) Integration Testing with JavaScript
[write](https://github.com/john-cd/rust_howto/issues/1217) Compiling Rust to WASM
[write](https://github.com/john-cd/rust_howto/issues/1217) Leveraging SIMD in WASM
[write](https://github.com/john-cd/rust_howto/issues/1217) Profiling Rust-WASM Applications
</div>
