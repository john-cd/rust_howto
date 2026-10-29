<!-- TODO Reviewed ??-??-2026 -->

# WebAssembly Runtimes

{{#include wasm_runtimes.incl.md}}

Standalone runtimes execute WebAssembly outside a browser. Wasmtime and Wasmer offer both embedding APIs and command-line tools; smaller interpreters serve different footprint and deployment needs. Choose the required instruction proposals, WASI interfaces, host platforms, and resource limits before selecting a runtime. A compiler backend such as Cranelift is not a complete runtime.

## Compare Runtimes

This support snapshot was reviewed on October 29, 2026. Core Wasm execution, WASI host imports, and the component model are separate capabilities; support for one does not imply the others.

| Runtime                      | Implementation and embedding API                                | WASI and components                                                                                                                                         | Maintenance and fit                                                                            |
| ---------------------------- | --------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| [Wasmtime][runtime~wasmtime] | Rust; Rust and C embedding APIs, JIT/AOT compilation            | `wasmtime-wasi` implements host interfaces; native component-model API with WIT bindings                                                                    | Active Bytecode Alliance project; configurable server/plugin runtime.                          |
| [Wasmer][runtime~wasmer]     | Rust; Rust and C APIs with selectable compiler backends         | WASI support is provided separately through Wasmer's WASIX integration; check required interfaces rather than assuming Wasmtime component API compatibility | Active project; compiler and host target choices affect portability.                           |
| [Wasmi][runtime~wasmi]       | Rust interpreter; Rust/C APIs, `no_std` configurations          | WASI Preview 1 through `wasmi_wasi`; do not assume a core module is a component                                                                             | Active interpreter focused on deterministic execution and embedded/plugin use.                 |
| [Wasm3][runtime~wasm3]       | C interpreter; C API and language bindings including Rust       | WASI subset; check the supported API list and proposals for the selected build                                                                              | Upstream explicitly reports minimal maintenance; evaluate that constraint for new deployments. |
| [WasmEdge][runtime~wasmedge] | C++; C API and Rust SDK, interpreter/AOT options                | WASI plus optional extensions/plugins; consult the current proposal matrix for component requirements                                                       | Active cloud/edge runtime; native installation and selected plugins matter.                    |
| [WAMR][runtime~wamr]         | C; C embedding API and Rust bindings; interpreter/AOT/JIT modes | Configurable WASI libc, threads, and other extensions; select interfaces explicitly                                                                         | Active small-footprint runtime; build configuration controls features and size.                |

## Embed Wasmer {#wasmer}

[![wasmer][c~wasmer~docs~badge]][c~wasmer~docs]

The optional `wasmer` feature uses Wasmer 7.5's single-pass compiler. This example compiles inline WAT, instantiates a module with no imports, and calls a typed export. Its assertion checks the guest result rather than merely reporting successful instantiation.

```rust,editable
{{#include ../../../crates/cats/wasm/examples/wasm_standalone_runtimes/wasmer.rs:example}}
```

From `later/crates`, run `cargo test -p wasm --example wasm_standalone_runtimes --features wasmer`. It needs no files, network access, or WASI host implementation. Wasmer's single-pass backend does not build on Windows, so run the example on Linux or macOS (or in WSL), or select another backend such as V8.

## Embed Wasmtime {#wasmtime}

[![wasmtime][c~wasmtime~docs~badge]][c~wasmtime~docs]

Wasmtime's `Engine` owns compilation configuration, `Module` holds compiled core Wasm, `Linker` supplies host imports, and `Store` holds execution state. The recipe uses inline WAT and checks the result of an exported addition function. Its explicit host callback is the only capability imported by that module.

```rust,editable
{{#include ../../../crates/cats/wasm/examples/wasm_standalone_runtimes/wasmtime.rs:example}}
```

Enable the optional `wasmtime` feature to compile and test this recipe. To execute a Rust command with Wasmtime's CLI, install the `wasm32-wasip1` target and compile the command for that target; a browser-targeted `wasm32-unknown-unknown` artifact does not automatically gain WASI imports.

```sh
rustup target add wasm32-wasip1
rustc hello.rs --target wasm32-wasip1
wasmtime hello.wasm
```

## Bound Guest Resources {#wasmtime-sandbox}

Enable fuel accounting to trap bounded computations that exhaust their budget. Configure `StoreLimits` for linear memory, tables, and instance counts. Epoch interruption offers a different, externally advanced interruption mechanism. No WASI imports are linked by the following sandbox, so it exposes no filesystem or network access.

```rust,editable
{{#include ../../../crates/cats/wasm/examples/wasm_standalone_runtimes/wasmtime_sandbox.rs:example}}
```

These controls complement runtime updates and process isolation. A guest can consume whatever resources the host permits; never treat basic memory isolation as a complete resource policy.

## WASI and the Component Model {#wasi}

WASI defines interfaces that a host implements for its guests. The `wasi` crate describes guest-side bindings; it is not by itself a universal host implementation for every runtime. Match the guest's target and interface versions to the embedding runtime and its host library.

The [Wasmtime component API][runtime~component] uses `Component`, a component-specific `Linker`, and `wasmtime::component::bindgen!` to generate Rust bindings from a WIT world. This API differs from the core `Module` and `Linker` used above. Use it when the application needs typed component interfaces rather than inventing a core-Wasm function ABI.

## Related Topics {#related-topics .skip}

- [WebAssembly overview](index.md) and [development tools](wasm_development.md).
- [Cloud development](../../other/cloud/rust_native_cloud_development.md).
- [Virtualization](../virtualization/index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [Sandbox example follow-up](https://github.com/john-cd/rust_howto/issues/1926).
</div>
