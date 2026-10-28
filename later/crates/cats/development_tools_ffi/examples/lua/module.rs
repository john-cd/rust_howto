// ANCHOR: example
//! Exports a Rust function as a loadable Lua module with `mlua`'s `module`
//! feature.

use mlua::prelude::*;

/// Builds the table returned when Lua loads this module.
fn module_exports(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set(
        "double",
        lua.create_function(|_, value: i64| Ok(value * 2))?,
    )?;
    Ok(exports)
}

/// Provides the module entry point called by Lua's `require` function.
#[mlua::lua_module]
fn lua_module(lua: &Lua) -> LuaResult<LuaTable> {
    module_exports(lua)
}

// ANCHOR_END: example
