#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! The `main` function, the entry point of a binary crate.

// `main` takes no parameters and, in this case,
// implicitly returns the "unit type" (),
// similar to 'void' in C/C++/Java.
/// Prints a greeting.
fn main() {
    println!("Hello, world!");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the main fn example through its entry point.
    #[test]
    fn test_main_fn_example_entry_point() {
        main();
    }
}
