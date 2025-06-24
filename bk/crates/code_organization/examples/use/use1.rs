#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use the `use` keyword to bring items into
//! scope without having to write their full path.

/// Modules are used to organize code into logical units.
mod a {
    /// Nested module.
    /// We prefix `mod b` with `pub` to make it public.
    pub mod b {
        /// Public function.
        pub fn fn_in_b() {
            println!("in b!");
        }

        pub struct StructInB;
    }

    pub const CONST_IN_A: u8 = 1;
    pub type Point = (u8, u8);
}

// Bring the `b` module in scope with the `use` keyword.
use a::b;

// A relative path starts from the current module and uses
// `self`, or an identifier in the current module.
// You could also write: `use self::a::b;`.

fn main() {
    // Call the function using its full path.
    a::b::fn_in_b();

    // Call the function using its shortened path.
    b::fn_in_b();

    // Use declarations can exist inside of functions.
    // Bring the `StructInB` struct in scope, then refer to it via its short
    // name:
    use a::b::StructInB;
    let _ = StructInB;

    // Use declarations work for constants, type aliases, enums, statics,
    // traits, etc.
    use a::CONST_IN_A;
    let _ = CONST_IN_A;

    use a::Point;
    let _p: Point = (1, 2);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
