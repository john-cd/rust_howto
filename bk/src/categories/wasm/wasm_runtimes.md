# WebAssembly Runtimes

{{#include wasm_runtimes.incl.md}}

## `wasmer` {#wasmer}

[![wasmer][c-wasmer-badge]][c-wasmer]{{hi:wasmer}}
[![wasmer-crates.io][c-wasmer-crates.io-badge]][c-wasmer-crates.io]
[![wasmer-github][c-wasmer-github-badge]][c-wasmer-github]
[![wasmer-lib.rs][c-wasmer-lib.rs-badge]][c-wasmer-lib.rs]
[![cat-wasm][cat-wasm-badge]][cat-wasm]{{hi:WebAssembly}}

[`wasmer`][wasmer-website]{{hi:wasmer.io}}⮳ is a runtime for executing WebAssembly on the server-side. It supports JIT (Just In Time), AOT (Ahead Of Time) compilation, an experimental interpreter, as well as pluggable compilers. Install with:

```sh
curl https://get.wasmer.io -sSfL | sh
```

Test with:

```sh
wasmer run cowsay "hello world"
```

You can embed the Wasmer runtime in your code with the Wasmer SDK:

```rust,editable
{{#include ../../../crates/cats/wasm/tests/wasmer.rs:example}}
```

## `wasmtime` {#wasmtime}

[![wasmtime][c-wasmtime-badge]][c-wasmtime]{{hi:wasmtime}}
[![wasmtime-crates.io][c-wasmtime-crates.io-badge]][c-wasmtime-crates.io]
[![wasmtime-github][c-wasmtime-github-badge]][c-wasmtime-github]
[![wasmtime-lib.rs][c-wasmtime-lib.rs-badge]][c-wasmtime-lib.rs]
[![cat-wasm][cat-wasm-badge]][cat-wasm]{{hi:WebAssembly}}

Wasmtime is a lightweight WebAssembly runtime that is fast, secure, and standards-compliant. Wasmtime runs WebAssembly code outside of the Web, and can be used both as a command-line utility or as a library embedded in a larger application.

Install the [`wasmtime`][c-wasmtime]⮳{{hi:wasmtime}} CLI with:

```sh
curl https://wasmtime.dev/install.sh -sSf | bash
```

Compile your Rust source code (here, `hello.rs`) to [WASM][p-wasm], then execute it with [`wasmtime`][c-wasmtime]⮳{{hi:wasmtime}}:

```sh
rustup target add wasm32-wasip1
rustc hello.rs --target wasm32-wasip1
wasmtime hello.wasm
```

[docs.wasmtime.dev][c-wasmtime-docs]⮳

The [`wasmtime`][c-wasmtime-github]⮳ crate is a high-level API to expose the Wasmtime runtime. It is useful for embedding WebAssembly into your code and interacting with WebAssembly modules or WebAssembly components. For example, you can compile WebAssembly, create instances, and call functions.

```rust,editable
{{#include ../../../crates/cats/wasm/tests/wasmtime.rs:example}}
```

## See also

[rustwasm.github.io][rustwasm-website]{{hi:rustwasm}}⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[others: organize/write (P2)](https://github.com/john-cd/rust_howto/issues/496)

[others: cover (P2)](https://github.com/john-cd/rust_howto/issues/497)
[websocket: cover others e.g. async_std_tungstenite (P2)](https://github.com/john-cd/rust_howto/issues/521)

- wasm-bindgen
- wasm-pack

## Rust Crates for WASM Runtimes

This table focuses specifically on Rust crates that can be used as WebAssembly (WASM) runtimes, meaning they can execute WASM bytecode. This is distinct from tools used to *compile* to [WASM][p-wasm] (like [`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}}).

The [`wasmtime`][c-wasmtime]⮳{{hi:wasmtime}}  runtime, backed by Mozilla, has become very popular and is a strong default choice for many projects.

| Runtime Crate | Description | Key Features | WASI Support | Embeddable | Notes |
|---|---|---|---|---|---|
| [`wasmi`][c-wasmi]⮳{{hi:wasmi}} | WASM interpreter written in Rust. | Interprets WASM bytecode, good for sandboxing and portability. | Yes (via [`wasi-rs`][c-wasi]⮳{{hi:wasi-rs}}) | Yes | A popular and actively maintained WASM interpreter. |
| [`wasmtime`][c-wasmtime]⮳{{hi:wasmtime}} | A fast and secure runtime for WebAssembly, developed by Mozilla. | Ahead-of-time (AOT) compilation for performance, supports WASI. | Yes | Yes | Focuses on speed and security. Used in various projects. |
| [`cranelift`][c-cranelift]⮳{{hi:cranelift}} (used by `wasmtime`) | A code generator used by [`wasmtime`][c-wasmtime]⮳{{hi:wasmtime}} for AOT compilation. | Generates optimized machine code from WASM. | N/A (part of [`wasmtime`][c-wasmtime]⮳{{hi:wasmtime}}) | N/A | Not a runtime itself, but a crucial component of [`wasmtime`][c-wasmtime]⮳{{hi:wasmtime}} . |
| [`wasi-rs`][c-wasi]⮳{{hi:wasi-rs}} | Bindings and utilities for the WebAssembly System Interface (WASI). | Provides an API for WASM modules to interact with the host environment (file system, networking, etc.). | Yes | Yes | Essential for running WASM outside of the browser. Used by [`wasmi`][c-wasmi]⮳{{hi:wasmi}} and [`wasmtime`][c-wasmtime]⮳{{hi:wasmtime}} . |
| `lucet` (mostly deprecated) | A cloud-native WebAssembly runtime. | Designed for serverless and cloud environments. | Yes | Yes | Largely superseded by [`wasmtime`][c-wasmtime]⮳{{hi:wasmtime}} . Still relevant for some legacy systems. |
| [`rwasm`][c-rwasm]⮳{{hi:rwasm}} | A lightweight WASM runtime. | Small footprint, suitable for embedded systems. | Yes | Yes | Designed for resource-constrained environments. |

## Key Considerations

- Interpreter vs. AOT Compilation: [`wasmi`][c-wasmi]⮳{{hi:wasmi}} is an interpreter, while [`wasmtime`][c-wasmtime]⮳{{hi:wasmtime}}  uses AOT compilation. AOT compilation generally leads to faster execution but may have a longer startup time.
- WASI Support: WASI is crucial for running [WASM][p-wasm] outside of the browser, and all the listed runtimes support it.
- Embeddability: Most of these runtimes are designed to be embeddable into other applications.
- Performance: [`wasmtime`][c-wasmtime]⮳{{hi:wasmtime}}  is generally considered to be very performant due to its AOT compilation.
- Security: WASM runtimes often prioritize security, as WASM is often used to execute untrusted code.

## Choosing a Runtime

- General Purpose/Sandboxing: [`wasmi`][c-wasmi]⮳{{hi:wasmi}} is a good choice.
- Performance: [`wasmtime`][c-wasmtime]⮳{{hi:wasmtime}}  is highly recommended.
- Cloud/Serverless: [`wasmtime`][c-wasmtime]⮳{{hi:wasmtime}}  is the most common choice.
- [Embedded][p-embedded] Systems: [`rwasm`][c-rwasm]⮳{{hi:rwasm}} is designed for resource-constrained environments.

</div>
