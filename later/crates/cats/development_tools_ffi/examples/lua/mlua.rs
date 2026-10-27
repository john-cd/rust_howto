#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Creates a Lua table in Rust and reads it from an embedded Lua 5.4 runtime.

use mlua::prelude::*;

// In standalone mode, `mlua` allows to add scripting support.
/// Creates a Lua table from Rust and iterates over it in Lua.
fn main() -> LuaResult<()> {
    let lua = Lua::new();

    let map_table = lua.create_table()?;
    map_table.set(1, "one")?;
    map_table.set("two", 2)?;

    lua.globals().set("map_table", map_table)?;

    lua.load("for k,v in pairs(map_table) do print(k,v) end")
        .exec()?;

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> LuaResult<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that Rust-created values are available to the Lua runtime.
    /// Runs the mlua example through its entry point.
    #[test]
    fn test_mlua_example_entry_point() -> LuaResult<()> {
        main()?;
        Ok(())
    }
}
