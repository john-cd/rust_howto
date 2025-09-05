# WebAssembly Runtimes

{{#include wasm_runtimes.incl.md}}

## WASM Runtimes {#wasm-runtimes .skip}

This table focuses on Rust crates that can be used as WebAssembly (WASM) runtimes, meaning they can execute [WASM][p~wasm] bytecode. This is distinct from tools used to *compile* to [WASM][p~wasm].

| Runtime Crate | Description | Key Features | WASI Support | Embeddable | Notes |
|---|---|---|---|---|---|
| [`wasmi`][c~wasmi~docs]↗{{hi:wasmi}} | WASM interpreter written in Rust. | Interprets WASM bytecode, good for sandboxing and portability. | Yes (via [`wasi-rs`][c~wasi~docs]↗{{hi:wasi-rs}}) | Yes | A popular and actively maintained WASM interpreter. |
| [`wasmtime`][c~wasmtime~docs]↗{{hi:wasmtime}} | A fast and secure runtime for WebAssembly, developed by Mozilla. | Ahead-of-time (AOT) compilation for performance, supports WASI. | Yes | Yes | Focuses on speed and security. |
| [`cranelift`][c~cranelift~docs]↗{{hi:cranelift}} (used by [`wasmtime`][c~wasmtime~docs]↗{{hi:wasmtime}}) | A code generator used by [`wasmtime`][c~wasmtime~docs]↗{{hi:wasmtime}} for AOT compilation. | Generates optimized machine code from WASM. | N/A (part of [`wasmtime`][c~wasmtime~docs]↗{{hi:wasmtime}}) | N/A | Not a runtime itself, but a crucial component of [`wasmtime`][c~wasmtime~docs]↗{{hi:wasmtime}}. |
| [`rwasm`][c~rwasm~docs]↗{{hi:rwasm}} | A lightweight WASM runtime. | Small footprint, suitable for embedded systems. | Yes | Yes | Designed for resource-constrained environments. |

## Choosing a Runtime {#choosing-a-runtime .skip}

The [`wasmtime`][c~wasmtime~docs]↗{{hi:wasmtime}} runtime, backed by Mozilla, has become very popular and is a strong default choice for many projects.

| Purpose | Recommendation |
|---|---|
| Performance | [`wasmtime`][c~wasmtime~docs]↗{{hi:wasmtime}} is highly recommended. |
| Sandboxing | [`wasmi`][c~wasmi~docs]↗{{hi:wasmi}} is a good choice. |
| Cloud/Serverless | [`wasmtime`][c~wasmtime~docs]↗{{hi:wasmtime}} is the most common choice. |
| [Embedded][p~embedded] Systems | [`rwasm`][c~rwasm~docs]↗{{hi:rwasm}} is designed for resource-constrained environments. |

Notes:

- [`wasmi`][c~wasmi~docs]↗{{hi:wasmi}} is an interpreter, while [`wasmtime`][c~wasmtime~docs]↗{{hi:wasmtime}} uses AOT compilation. AOT compilation generally leads to faster execution but may have a longer startup time.
- WASI support is crucial for running [WASM][p~wasm] outside of the browser, and all the listed runtimes support it.
- Most of these runtimes are designed to be embeddable into other applications.
- [`wasmtime`][c~wasmtime~docs]↗{{hi:wasmtime}} is generally considered to be very performant due to its AOT compilation.
- WASM runtimes often prioritize security, as [WASM][p~wasm] is often used to execute untrusted code.

## Code Examples {#code-examples .skip}

### `wasmer` {#wasmer}

[![wasmer][c~wasmer~docs~badge]][c~wasmer~docs]{{hi:wasmer}}
[![wasmer~crates.io][c~wasmer~crates.io~badge]][c~wasmer~crates.io]
[![wasmer~repo][c~wasmer~repo~badge]][c~wasmer~repo]
[![wasmer~lib.rs][c~wasmer~lib.rs~badge]][c~wasmer~lib.rs]
[![cat~wasm][cat~wasm~badge]][cat~wasm]{{hi:WebAssembly}}

[`wasmer`][wasmer~website]↗{{hi:wasmer.io}} is a runtime for executing WebAssembly on the server-side. It supports JIT (Just In Time), AOT (Ahead Of Time) compilation, an experimental interpreter, as well as pluggable compilers. Install with:

```sh
curl https://get.wasmer.io -sSfL | sh
```

Test with:

```sh
wasmer run cowsay "hello world"
```

You can embed the Wasmer runtime in your code with the Wasmer SDK:

```rust,editable
{{#include ../../../crates/cats/wasm/examples/wasm_standalone_runtimes/wasmer.rs:example}}
```

### `wasmtime` {#wasmtime}

[![wasmtime][c~wasmtime~docs~badge]][c~wasmtime~docs]{{hi:wasmtime}}
[![wasmtime~crates.io][c~wasmtime~crates.io~badge]][c~wasmtime~crates.io]
[![wasmtime~repo][c~wasmtime~repo~badge]][c~wasmtime~repo]
[![wasmtime~lib.rs][c~wasmtime~lib.rs~badge]][c~wasmtime~lib.rs]
[![cat~wasm][cat~wasm~badge]][cat~wasm]{{hi:WebAssembly}}

[`wasmtime`][c~wasmtime~docs]↗{{hi:wasmtime}} is a lightweight WebAssembly runtime that is fast, secure, and standards-compliant. Wasmtime runs WebAssembly code outside of the Web, and can be used both as a command-line utility or as a library embedded in a larger application.

Install the [`wasmtime`][c~wasmtime~docs]↗{{hi:wasmtime}} CLI with:

```sh
curl https://wasmtime.dev/install.sh -sSf | bash
```

Compile your Rust source code (here, `hello.rs`) to [WASM][p~wasm], then execute it with [`wasmtime`][c~wasmtime~docs]↗{{hi:wasmtime}}:

```sh
rustup target add wasm32-wasip1
rustc hello.rs --target wasm32-wasip1
wasmtime hello.wasm
```

[docs.wasmtime.dev][c~wasmtime~docs]↗.

The [`wasmtime`][c~wasmtime~repo]↗ crate is a high-level API to expose the Wasmtime runtime. It is useful for embedding WebAssembly into your code and interacting with WebAssembly modules or WebAssembly components. For example, you can compile WebAssembly, create instances, and call functions.

```rust,editable
{{#include ../../../crates/cats/wasm/examples/wasm_standalone_runtimes/wasmtime.rs:example}}
```

## WebAssembly System Interface (WASI) {#wasi.skip}

[`wasi-rs`][c~wasi~docs]↗{{hi:wasi-rs}} provides bindings for the WASI API, enabling your WASM code to interact with the host environment (file system, networking, etc.). In particular, WASI allows you to run WASM code on servers or other environments outside the browser. Used by [`wasmi`][c~wasmi~docs]↗{{hi:wasmi}} and [`wasmtime`][c~wasmtime~docs]↗{{hi:wasmtime}}.

[`cargo-wasi`][c~cargo-wasi~docs]↗{{hi:cargo-wasi}} is used for compiling to WASI (WebAssembly System Interface), which is useful for running WASM outside of the browser (e.g., on servers or embedded devices).

## See Also {#see-also .skip}

- [rustwasm.github.io][rustwasm~website]↗{{hi:rustwasm}}.

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[others: organize/write](https://github.com/john-cd/rust_howto/issues/496) need full review
cover [`wasmer`][c~wasmer~docs]↗{{hi:wasmer}} in intro
</div>
