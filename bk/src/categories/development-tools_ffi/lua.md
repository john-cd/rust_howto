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
</div>
