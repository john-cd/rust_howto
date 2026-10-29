// TODO Reviewed ??-??-2026.

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

    #[test]
    fn globals_round_trip_through_lua() -> LuaResult<()> {
        let lua = Lua::new();
        let table = lua.create_table()?;
        table.set(1, "one")?;
        table.set("two", 2)?;
        lua.globals().set("map_table", table)?;

        let one: String = lua.load("return map_table[1]").eval()?;
        let two: i64 = lua.load("return map_table.two").eval()?;
        assert_eq!(one, "one");
        assert_eq!(two, 2);
        Ok(())
    }

    #[test]
    fn lua_syntax_errors_are_reported() {
        let lua = Lua::new();
        assert!(lua.load("this is not lua").exec().is_err());
    }
}
