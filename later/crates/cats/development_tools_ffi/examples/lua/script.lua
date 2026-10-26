-- Purpose: Load and use the Lua module implemented in Rust.
-- Usage: Run with lua later/crates/cats/development_tools_ffi/examples/lua/script.lua from the example directory.
--
-- TODO Reviewed ??-??-2026

require("my_module")

print(hello()) -- Output: Hello from Rust!

print(my_module.value) -- Output: 42
