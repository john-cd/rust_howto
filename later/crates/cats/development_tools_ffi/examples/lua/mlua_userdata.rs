// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Exposes a Rust counter as Lua userdata with a method, using vendored Lua.
//!
//! Userdata is a Rust value that Lua code holds as an opaque object. Methods
//! registered on it let scripts call back into Rust, while the data stays in
//! Rust and is owned and protected by it.

use mlua::prelude::*;

/// A counter whose value is owned by Rust and exposed to Lua.
struct Counter {
    /// Value stored by this wrapper.
    value: i64,
}

impl LuaUserData for Counter {
    /// Registers methods that Lua scripts can call on the counter.
    ///
    /// # Arguments
    /// * `methods` - Mutable m whose state is updated by this operation.
    ///
    /// # Errors
    /// Returns an error if a fallible operation cannot complete.
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("increment", |_, this, amount: i64| {
            this.value += amount;
            Ok(this.value)
        });
    }
}

/// Creates userdata and calls its method from Lua.
///
/// # Errors
/// An error is returned if create_userdata, globals fails.
fn main() -> LuaResult<()> {
    let lua = Lua::new();
    let counter = lua.create_userdata(Counter { value: 40 })?;
    lua.globals().set("counter", counter)?;

    let value: i64 = lua.load("return counter:increment(2)").eval()?;
    println!("counter = {value}");
    assert_eq!(value, 42);
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> LuaResult<()> {
    main()
}

/// value.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the Lua counter example and verifies the incremented value.
    ///
    /// # Errors
    /// Returns an error if a fallible operation cannot complete.
    #[test]
    fn lua_calls_counter_method() -> LuaResult<()> {
        main()
    }
}
