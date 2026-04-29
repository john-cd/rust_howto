# Plugin Architecture

A plugin architecture lets a host application support extensions that are developed and deployed separately from the core program. In Rust, this usually means balancing safety, portability, and runtime flexibility.

## Create a Plugin System {#plugins}

[![std][c~std~docs~badge]][c~std~docs]

```rust,editable
{{#include ../../crates/standard_library/examples/any/plugin.rs:example}}
```

For a true plugin architecture where plugins are compiled as separate shared libraries (`.so` on Linux, `.dll` on Windows, `.dylib` on macOS) and loaded at runtime, you would typically:

- Use the [`libloading`][c~libloading~crates.io]↗{{hi:libloading}} crate, which provides safe FFI (Foreign Function Interface) wrappers to dynamically load shared libraries and resolve symbols.
- Define a C-compatible ABI: Because Rust's internal ABI is not stable across different compiler versions, you should define your plugin interface using [`#[repr(C)]`][book~rust-reference~c-representation]↗{{hi:repr}} structs and [`extern "C"`][keyword~extern]↗{{hi:extern "C"}} functions.
- Define an entry point: Each plugin [`.so`][.so-files]↗{{hi:.so}}/[`.dll`][.dll-files]↗{{hi:.dll}} would export a specific [`extern "C"`][keyword~extern]↗{{hi:extern "C"}} function that the host calls to obtain a `Box<dyn Plugin>`.
- Manage versions carefully: Even with `extern "C"`, you need robust versioning for the shared plugin interface crate. Crates like [`abi_stable`][c~abi_stable~docs]↗{{hi:abi_stable}} can help provide compatibility guarantees.

A dynamic plugin system is powerful, but it also introduces complexity. If your application does not need plugins to be loaded after deployment, a static plugin registry with a shared trait object interface is often easier to implement and maintain.

## WASM-based Plugin Systems

An alternative to loading shared libraries is using **WebAssembly (WASM)**. This approach offers several advantages:

- **Security**: Plugins run in a sandbox, preventing them from accessing the host's memory or file system directly.
- **Portability**: The same WASM plugin can run on any platform.
- **Language Independence**: Plugins can be written in any language that compiles to WASM (for example, C, C++, AssemblyScript, or Zig).

Crates like [`wasmtime`][c~wasmtime~docs]↗{{hi:wasmtime}} or [`wasmer`][c~wasmer~docs]↗{{hi:wasmer}} are commonly used to host WASM-based plugin systems in Rust.

## When to Use Dynamic Plugins

- Use dynamic plugins when you need third-party extensions loaded after deployment.
- Prefer a static registration or compile-time plugin architecture when safety, performance, and portability are more important than runtime extensibility.

## Related Topics {#related-topics .skip}

- [[dynamic_typing | Dynamic Typing]].
- [[wasm | WebAssembly]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
