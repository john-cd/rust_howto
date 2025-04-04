// // ANCHOR: example
// //! Example of creating a Lua module using `mlua`.
// //! In module mode, `mlua` allows to create a compiled Lua module that can
// //! be loaded from Lua code using `require`. In this case, `mlua` uses an
// //! external Lua runtime.
// //!
// //! 1. Add the following to your `Cargo.toml` file:
// //! ```toml
// //! [lib]
// //! crate-type = ["cdylib"]
// //!
// //! [dependencies]
// //! mlua = { version = "0.10.2", features = ["lua54", "module"] }
// //! ```
// //!
// //! 2. Run the Lua script:
// //! - Compile the Rust code: `cargo build --release`
// //! - Run the Lua script using a Lua interpreter: `lua script.lua`

// use mlua::prelude::*;

// /// Prints a greeting to the console.
// ///
// /// # Arguments
// ///
// /// * `_` - The Lua context.
// /// * `name` - The name to greet.
// fn hello(_: &Lua, name: String) -> LuaResult<()> {
//     println!("hello, {}!", name);
//     Ok(())
// }

// /// Creates a Lua module with a function `hello`.
// ///
// /// This function is marked as a Lua module using the `#[mlua::lua_module]`
// attribute. #[mlua::lua_module]
// fn my_module(lua: &Lua) -> LuaResult<LuaTable> {

//     let exports = lua.create_table()?;
//     exports.set("hello", lua.create_function(hello)?)?;
//     Ok(exports)

//     // // Gets the global table
//     // let globals = lua.globals();

//     // // Creates a Lua function and register it in the global table
//     // // with the name "hello".
//     // globals.set(
//     //     "hello2",
//     //     lua.create_function(|_, _: ()| Ok("Hello from
// Rust!".to_owned()))?,     // )?; }
// // ANCHOR_END: example

// // [finish](https://github.com/john-cd/rust_howto/issues/1031)
