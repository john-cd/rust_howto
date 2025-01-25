// // ANCHOR: example
// use mlua::prelude::*;

// // In a module mode, `mlua` allows to create a compiled Lua module that can
// be // loaded from Lua code using require. In this case mlua uses an external
// Lua // runtime

// // Add the following to your `Cargo.toml` file:
// // [lib]
// // crate-type = ["cdylib"]
// //
// // [dependencies]
// // mlua = { version = "0.10.2", features = ["lua54", "module"] }

// // Run the Lua script:
// // - Compile the Rust code: `cargo build --release`
// // - Run the Lua script using a Lua interpreter: `lua script.lua`

// fn hello(_: &Lua, name: String) -> LuaResult<()> {
//     println!("hello, {}!", name);
//     Ok(())
// }

// #[mlua::lua_module] // Mark the my_module function as a Lua module.
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
// Rust!".to_owned()))?,     // )?;
// }
// // ANCHOR_END: example

// // TODO P1 write; add to markdown
