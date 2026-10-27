#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Importing items from the standard library with `use`.

// This `use` statement allows us to use `HashMap` directly without having to
// specify its full path.
// IDIOM: import the `struct` itself, not its parent module.
use std::collections::HashMap;

// `HashMap` is declared in the standard library (`std`), which is an external
// crate. We therefore prefix the path with the crate's name.

/// Uses an imported `HashMap`.
fn main() {
    // We now refer to `HashMap` by its name.
    let mut mymap: HashMap<u32, String> = HashMap::new();

    // Let's add something to it then print...
    mymap.entry(42).or_insert("my favorite number".into());
    println!("{mymap:?}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the use3 example through its entry point.
    #[test]
    fn test_use3_example_entry_point() {
        main();
    }
}
