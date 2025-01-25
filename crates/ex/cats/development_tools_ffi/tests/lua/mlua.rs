// ANCHOR: example
use mlua::prelude::*;

// `mlua` provides bindings to Lua programming language for Rust

// By default mlua does not enable any features.
// For example, add the following to your `Cargo.toml` file to enable Lua 5.4
// support: ```toml
// [dependencies]
// mlua = { version = "0.10.2", features = ["lua54", "vendored"] }
// ```

// In standalone mode mlua allows to add to your application scripting support
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

#[test]
fn test() -> LuaResult<()> {
    main()?;
    Ok(())
}
// TODO P1 write; add to markdown
