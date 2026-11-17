<!-- TODO Reviewed ??-??-2026. -->

# Plugin Architecture

{{#include plugin.incl.md}}

A plugin architecture ranges from a set of implementations linked into one program to third-party code loaded after deployment. Pick the boundary first: it determines the interface, trust model, and compatibility guarantees you need.

## Build an In-Process Plugin Host {#in-process-plugin-host}

This example uses trait objects to dispatch among plugins already linked into the host. Its `Any`-based context stores services by concrete type, so a plugin can retrieve a service without the host and plugin sharing a more specific context type. It demonstrates in-process extension points; it does not load a separate library at runtime.

[![std][c~std~docs~badge]][c~std~docs]

```rust,noplayground
{{#include ../../../crates/standard_library/examples/any/plugin.rs:example}}
```

For an application that only needs independently developed components to register with the host at compile time, an in-process registry is usually simpler and safer than dynamic loading. See [explicit and inventory-based plugin registration](../rust-patterns/plugin_registration.md).

## Load Native Plugins at Runtime

Native shared libraries execute with the host process's privileges. Load only code you trust. A library loader such as [`libloading`][c~libloading~docs]↗ exposes an unsafe boundary: the host must use the correct symbol types, keep the library loaded while its symbols or plugin values are in use, and agree on allocation and destruction.

Rust's native ABI is not stable across compiler versions. `#[repr(C)]` can define the layout of C-compatible data, but it does not make Rust-specific types such as `Box<dyn Plugin>` safe to pass across a C ABI. A native interface should use a deliberately versioned C ABI, C-compatible arguments, opaque handles, and explicit creation and destruction functions. Do not allow Rust panics to unwind across the ABI boundary. A project that needs a Rust-to-Rust stable ABI can use a framework such as [`abi_stable`](https://docs.rs/abi_stable)↗ instead of inventing one.

## Run WebAssembly Plugins

WebAssembly can provide a portable guest-host boundary, but the format alone does not guarantee a sandbox. The runtime and host decide which imports and capabilities the plugin receives and which resource limits apply. Grant only the access the plugin needs. [`Wasmtime`][c~wasmtime~docs]↗ and [`Wasmer`][c~wasmer~docs]↗ are Rust runtimes for WebAssembly.

## Related Topics {#related-topics .skip}

- [Dynamic Typing](../../standard-library/dynamic_typing.md).
- [Explicit and Inventory-Based Plugin Registration](../rust-patterns/plugin_registration.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
