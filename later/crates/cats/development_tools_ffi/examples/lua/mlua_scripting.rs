#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Calls an embedded Lua chunk and a Rust function while exposing only a
//! selected subset of Lua's standard libraries.

use mlua::prelude::*;

/// Evaluates a chunk with a Rust callback in a restricted standard library.
fn main() -> LuaResult<()> {
    let lua = Lua::new_with(
        LuaStdLib::MATH | LuaStdLib::STRING | LuaStdLib::TABLE,
        LuaOptions::default(),
    )?;
    let multiply =
        lua.create_function(|_, (left, right): (i64, i64)| Ok(left * right))?;
    lua.globals().set("multiply", multiply)?;
    lua.globals().set("dofile", None::<LuaFunction>)?;
    lua.globals().set("loadfile", None::<LuaFunction>)?;

    let result: i64 = lua.load("return multiply(6, 7)").eval()?;
    println!("Lua returned {result}");
    assert_eq!(result, 42);

    // The selected library set omits process and package access; file-loading
    // globals are cleared explicitly. This limits exposed capabilities, but
    // it is not a full sandbox.
    assert!(
        lua.globals()
            .get::<Option<LuaFunction>>("dofile")?
            .is_none()
    );
    assert!(lua.globals().get::<Option<LuaTable>>("os")?.is_none());
    assert!(
        lua.globals()
            .get::<Option<LuaFunction>>("require")?
            .is_none()
    );
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> LuaResult<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies Lua evaluation, Rust callbacks, and the restricted libraries.
    #[test]
    fn evaluates_lua_and_calls_rust() -> LuaResult<()> {
        main()
    }
}
