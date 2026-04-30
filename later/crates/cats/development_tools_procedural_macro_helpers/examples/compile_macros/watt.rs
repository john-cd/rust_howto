#![allow(dead_code)]
// ANCHOR: example
//! Watt is a procedural macro library that allows us to write our
//! procedural macros without the overhead of the `proc_macro` crate.
//! Watt is a runtime for executing Rust procedural macros compiled as
//! WebAssembly.
//!
//! Using Watt involves two main parts:
//! 1. A "proc-macro" crate that acts as a shim, loading and executing the
//!    WASM-compiled macro.
//! 2. A "source" crate that contains the actual macro logic and is compiled to
//!    WASM.
//!
//! This approach provides isolation and can speed up compilation as the macro
//! logic is pre-compiled to WASM.

use development_tools_procedural_macro_helpers::watt_macro;

// Demonstrating the usage of a macro that could be powered by Watt.
// In this example, 'watt_macro' is a shim that would normally load a WASM file.
watt_macro!(some input);

pub fn main() {
    println!("Demonstrating Watt-like macro usage:");
    watt_demo();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watt_demo() {
        main();
    }
}
// ANCHOR_END: example

// [finish](https://github.com/john-cd/rust_howto/issues/744)
