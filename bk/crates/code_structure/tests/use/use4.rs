#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use the `use` keyword to bring items from
//! external crates into scope, without having to write their full paths.

// Import a module from an external crate (here, `std`, the standard library).
// The absolute path begins with the crate name.
use std::array;
// Import a struct from an external crate.
// With the following, `HashMap` can be used without prefix in the current
// module.
use std::collections::HashMap;

fn main() {
    // Use a function from the `array` module of the `std` crate.
    let _arr = array::from_ref(&1);

    // Use `HashMap` from the `collections` module of the `std` crate.
    let _h: HashMap<String, String> = HashMap::new();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
