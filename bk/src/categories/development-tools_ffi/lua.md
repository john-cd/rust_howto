# Generate FFI bindings to Lua code

{{#include lua.incl.md}}

## `mlua` {#mlua}

[![mlua][c-mlua-badge]][c-mlua] [![mlua-crates.io][c-mlua-crates.io-badge]][c-mlua-crates.io] [![mlua-github][c-mlua-github-badge]][c-mlua-github] [![mlua-lib.rs][c-mlua-lib.rs-badge]][c-mlua-lib.rs]{{hi:mlua}}{{hi:Lua}}{{hi:Scripting}}{{hi:Async}}{{hi:LuaJIT}}{{hi:Luau}} [![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]{{hi:API bindings}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}

`mlua` provides high-level bindings to `Lua` 5.4/5.3/5.2/5.1 (including `LuaJIT`) and `Roblox Luau` with async/await features and support of writing native Lua modules in Rust.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/tests/lua/mlua.rs:example}}
```

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/tests/lua/mlua2.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P2 write](https://github.com/john-cd/rust_howto/issues/1073)

Interfacing with Lua from Rust is typically done using the Lua C API, and there are several Rust crates that provide bindings to it.

## Lua Bindings

rlua: A safe and high-level binding to Lua. It's a popular choice.
mlua: Another good option for Lua bindings. It focuses on being lightweight and efficient.
lua53: Direct, lower-level bindings to Lua 5.3. If you need very fine-grained control or are working with a specific Lua version, this might be an option. However, rlua or mlua are usually preferred for ease of use.

## Communication/Data Marshaling

(The Lua binding crates usually handle data marshaling between Rust and Lua types. You won't typically need separate serialization crates like serde in the same way as with some other FFI scenarios.)

## Build Tools

cargo: For building your Rust code.

## Key Concepts

Lua State: You'll work with a Lua state in your Rust code to interact with the Lua interpreter.
Calling Lua Functions from Rust: The binding crates provide ways to call Lua functions from Rust.
Calling Rust Functions from Lua: You can register Rust functions with Lua so that they can be called from Lua scripts.
Stack Manipulation (Lower-Level Bindings): If you use lower-level bindings, you might need to manipulate the Lua stack directly. Higher-level crates abstract this away.
rlua and mlua provide a much more ergonomic and safe way to interact with Lua compared to using the raw Lua C API directly.  They are the recommended starting points for Lua/Rust integration.

</div>
