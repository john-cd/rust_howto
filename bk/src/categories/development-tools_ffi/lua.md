<!-- TODO Reviewed ??-??-2026. -->

# Generate FFI Bindings to Lua Code

{{#include lua.incl.md}}

Interfacing with Lua from Rust is typically done using the Lua C API, and there are several Rust crates that provide bindings to it.

## `mlua` {#mlua}

{{#badges mlua api-bindings="API bindings" asynchronous=Asynchronous}}{{hi:Lua}}{{hi:Scripting}}{{hi:Async}}{{hi:LuaJIT}}{{hi:Luau}}

[`mlua`][c~mlua~docs]↗{{hi:mlua}} provides high-level bindings to Lua 5.5/5.4/5.3/5.2/5.1 (including `LuaJIT`) and Roblox Luau, with async/await support and the option to write native Lua modules in Rust.

The first three examples link against the system Lua 5.4 library. Each creates its own interpreter in the Rust process. Install the Lua 5.4 development files for your target before building. On Debian, `liblua5.4-dev` and `pkg-config` let Cargo find the library. On Windows with MSVC, use the import library and DLL from the same Lua 5.4 SDK.

For example, with an x64 MSVC Lua SDK installed under `C:\Tools\lua-5.4.9`, set the library directory, import library name, and runtime path in PowerShell. Run these commands from the repository root.

```powershell
$luaSdk = 'C:\Tools\lua-5.4.9'
$env:LUA_LIB = Join-Path $luaSdk 'lib'
$env:LUA_LIB_NAME = 'lua54'
$env:PATH = (Join-Path $luaSdk 'bin') + ';' + $env:PATH
just cargo test --manifest-path bk/platform/Cargo.toml --locked -p development_tools_ffi --features lua --example lua
```

Keep the library set small for scripts that should not access files or processes; the base library is always present, so the scripting example clears its file-loading globals explicitly. These choices limit capabilities but do not create a complete sandbox for hostile code. For untrusted scripts, use an isolated process or a runtime with a dedicated sandbox API.

```rust,noplayground
{{#include ../../../platform/cats/development_tools_ffi/examples/lua/mlua.rs:example}}
```

### Call Rust from Lua

Register Rust functions with `create_function`, evaluate a chunk with `load`, and select the standard libraries the script needs. This example omits file, process, and package libraries.

```rust,noplayground
{{#include ../../../platform/cats/development_tools_ffi/examples/lua/mlua_scripting.rs:example}}
```

### Expose Rust Userdata

Implement `UserData` to expose Rust-owned values and methods to scripts. The embedded interpreter owns the Lua value and checks its type when Lua calls Rust.

```rust,noplayground
{{#include ../../../platform/cats/development_tools_ffi/examples/lua/mlua_userdata.rs:example}}
```

### Build a Lua Module

Enable the `lua-module` feature and build the `lua_module` example as a dynamic library. This mode uses the interpreter that loads the library. The module and interpreter must use the same Lua ABI. On Windows, set `LUA_LIB` and `LUA_LIB_NAME` as above so the module links against the host's import library.

```powershell
just cargo build --manifest-path bk/platform/Cargo.toml --locked -p development_tools_ffi --features lua-module --example lua_module
```

Use the resulting `lua_module.dll` from the `debug/examples` directory under Cargo's configured target directory. From that directory, the Lua interpreter can load the DLL through `package.loadlib`. This calls the C entry point that `require` would find for a module named `lua_module`.

```lua
local open_module = assert(package.loadlib("./lua_module.dll", "luaopen_lua_module"))
local example = open_module()
assert(example.double(21) == 42)
```

```rust,noplayground
{{#include ../../../platform/cats/development_tools_ffi/examples/lua/module.rs:example}}
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
