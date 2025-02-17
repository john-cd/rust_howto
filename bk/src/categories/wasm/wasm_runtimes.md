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

Compile your Rust source code (here, `hello.rs`) to WASM, then execute it with [`wasmtime`][c-wasmtime]⮳{{hi:wasmtime}}:

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

</div>
