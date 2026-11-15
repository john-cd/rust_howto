// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Exports a Rust function as a loadable Lua module with `mlua`'s `module`
//! feature.

use mlua::prelude::*;

/// Builds the table returned when Lua loads this module.
///
/// # Arguments
/// * `lua` - Lua state that owns the returned table and function.
///
/// # Returns
/// The module table with its exported `double` function.
///
/// # Errors
/// Returns an error if Lua cannot allocate the table or function.
fn module_exports(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set(
        "double",
        lua.create_function(|_, value: i64| Ok(value * 2))?,
    )?;
    Ok(exports)
}

/// Provides the module entry point called by Lua's `require` function.
///
/// # Arguments
/// * `lua` - Lua state requesting this module.
///
/// # Returns
/// The module table returned to Lua's `require` function.
///
/// # Errors
/// Returns an error if module exports cannot be created.
#[mlua::lua_module]
fn lua_module(lua: &Lua) -> LuaResult<LuaTable> {
    module_exports(lua)
}
// ANCHOR_END: example

/// Creates the Lua module table returned by the loadable module.
///
/// # Errors
/// Returns an error if the example operation fails.
pub fn run() -> LuaResult<LuaTable> {
    lua_module(&Lua::new())
}

/// Checks that the module exports a working doubling function.
#[cfg(test)]
mod tests {
    use mlua::prelude::*;

    use super::module_exports;

    /// Doubles an integer passed through the exported Lua function.
    #[test]
    fn exported_double_function() {
        let lua = Lua::new();
        let exports = module_exports(&lua).expect("the module table can be created");
        let double: Function = exports
            .get("double")
            .expect("the module exports a `double` function");

        assert_eq!(double.call::<i64>(21).expect("doubling succeeds"), 42);
    }
}
