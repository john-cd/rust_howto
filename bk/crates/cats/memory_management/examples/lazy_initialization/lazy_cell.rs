#![allow(dead_code)]
// ANCHOR: example
//! Lazily initializes a single-threaded value with `std::cell::LazyCell`.

use std::cell::LazyCell;

/// Creates and reads a value initialized on first access.
fn main() {
    let message = LazyCell::new(|| String::from("initialized on first access"));
    assert_eq!(&*message, "initialized on first access");
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that `LazyCell` initializes its value on first access.
    #[test]
    fn initializes_on_first_access() {
        main();
    }
}
