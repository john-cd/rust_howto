#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to write `use` declarations to bring items
//! from external crates into scope, without having to write their full paths.

// Import a module from an external crate (here, `std`, the standard library),
// by writing a path that begins with the external crate name.
use std::array;
// That works as well for other items: types, traits, macros, etc.
// For example, import a struct from an external crate:
use std::collections::HashMap;

// With the above, `HashMap` can be used without prefix in the current
// module.

// You can prefix the path with `::`.
// This is useful to disambiguate between a local module and an external
// crate.
use ::anyhow::Result;

fn main() -> Result<()> {
    // Use a function from the `array` module of the `std` crate.
    // In this case, create an array of length 1 from a reference.
    let _arr = array::from_ref(&1);

    // Use `HashMap` from the `collections` module of the `std` crate.
    let _h: HashMap<String, String> = HashMap::new();

    // You can of course continue to refer to an item in an external crate by
    // its full path:
    let _h: HashMap<String, String> = std::collections::HashMap::new();

    Ok(())
}

/// A local module with the same name than an external crate.
mod anyhow {}
// ANCHOR_END: example

#[test]
fn test() -> Result<()> {
    main()?;
    Ok(())
}
