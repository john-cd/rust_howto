# Generate FFI Bindings to Lua Code

{{#include lua.incl.md}}

Interfacing with Lua from Rust is typically done using the Lua C API, and there are several Rust crates that provide bindings to it.

## `mlua` {#mlua}

[![mlua][c~mlua~docs~badge]][c~mlua~docs] [![mlua~crates.io][c~mlua~crates.io~badge]][c~mlua~crates.io] [![mlua~repo][c~mlua~repo~badge]][c~mlua~repo] [![mlua~lib.rs][c~mlua~lib.rs~badge]][c~mlua~lib.rs]{{hi:mlua}}{{hi:Lua}}{{hi:Scripting}}{{hi:Async}}{{hi:LuaJIT}}{{hi:Luau}} [![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

[`mlua`][c~mlua~docs]↗{{hi:mlua}} provides high-level bindings to `Lua` 5.4/5.3/5.2/5.1 (including `LuaJIT`) and `Roblox Luau` with async/await features and support of writing native Lua modules in Rust.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/lua/mlua.rs:example}}
```

## Communication/Data Marshaling {#communication-data-marshaling}

The Lua binding crates usually handle data marshaling between Rust and Lua types. You won't typically need separate serialization crates like [serde][p~serde] in the same way as with some other FFI scenarios.

## Build Tools {#build-tools}

Use [`cargo`][c~cargo~docs]↗{{hi:cargo}} for building your Rust code.

## Key Concepts {#key-concepts}

- Lua State: You'll work with a Lua state in your Rust code to interact with the Lua interpreter.
- Calling Lua Functions from Rust: The binding crates provide ways to call Lua functions from Rust.
- Calling Rust Functions from Lua: You can register Rust functions with Lua so that they can be called from Lua scripts.

[`mlua`][c~mlua~docs]↗{{hi:mlua}} provides a much more ergonomic and safe way to interact with Lua compared to using the raw Lua C API directly.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1073)
</div>
