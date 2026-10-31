<!-- TODO Reviewed ??-??-2026 -->

# Generate FFI Bindings to Lua Code

{{#include lua.incl.md}}

Interfacing with Lua from Rust is typically done using the Lua C API, and there are several Rust crates that provide bindings to it.

## `mlua` {#mlua}

{{#badges mlua api-bindings="API bindings" asynchronous=Asynchronous}}{{hi:Lua}}{{hi:Scripting}}{{hi:Async}}{{hi:LuaJIT}}{{hi:Luau}}

[`mlua`][c~mlua~docs]↗{{hi:mlua}} provides high-level bindings to Lua 5.5/5.4/5.3/5.2/5.1 (including `LuaJIT`) and Roblox Luau, with async/await support and the option to write native Lua modules in Rust.

The first three examples use standalone mode with Lua 5.4 and the `vendored` feature, so Cargo builds Lua from source and the Rust application owns the interpreter. Keep the library set small for scripts that should not access files or processes; the base library is always present, so the scripting example clears its file-loading globals explicitly. These choices limit capabilities but do not create a complete sandbox for hostile code. For untrusted scripts, use an isolated process or a runtime with a dedicated sandbox API.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/lua/mlua.rs:example}}
```

### Call Rust from Lua

Register Rust functions with `create_function`, evaluate a chunk with `load`, and select the standard libraries the script needs. This example omits file, process, and package libraries.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/lua/mlua_scripting.rs:example}}
```

### Expose Rust Userdata

Implement `UserData` to expose Rust-owned values and methods to scripts. The example uses the package's vendored Lua 5.4 configuration.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/lua/mlua_userdata.rs:example}}
```

### Build a Lua Module

Use `mlua`'s separate `module` feature when a Lua runtime should load the Rust `cdylib` with `require`. The module target is feature-gated in the example crate; module builds have different linking requirements from embedded applications, especially on Windows. Select either the `lua` feature (vendored standalone mode) or `lua-module` feature (external runtime module mode), because `mlua-sys` does not support both modes in one build.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/lua/module.rs:example}}
```

## Communication/Data Marshaling {#communication-data-marshaling}

The Lua binding crates usually handle data marshaling between Rust and Lua types. You won't typically need separate serialization crates like [serde][p~serde] in the same way as with some other FFI scenarios.

## Build Tools {#build-tools}

Use [`cargo`][c~cargo~docs]↗{{hi:cargo}} for building your Rust code.

## Key Concepts {#key-concepts}

- Lua State: A `Lua` value owns an interpreter state. These examples run embedded Lua in the Rust process.
- Calling Lua Functions from Rust: `load(...).eval()` returns a Lua value as a Rust type; `exec()` runs chunks whose result is not needed.
- Calling Rust Functions from Lua: You can register Rust functions with Lua so that they can be called from Lua scripts.

[`piccolo`][c~piccolo~docs]↗{{hi:piccolo}} is another Lua-family runtime implemented in Rust and may suit projects that need a different embedding model.

[`mlua`][c~mlua~docs]↗{{hi:mlua}} provides a much more ergonomic and safe way to interact with Lua compared to using the raw Lua C API directly.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
