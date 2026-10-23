#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! `OnceCell<T>` allows for single assignment, interior mutability.
//! It's typically used for lazy initialization of a value.

use std::cell::OnceCell;

/// Initializes a `OnceCell` once and reads it.
fn main() {
    let cell = OnceCell::new();

    // The cell is initially empty
    assert!(cell.get().is_none());

    // Initialize the cell
    let value = cell.get_or_init(|| {
        println!("Initializing...");
        "Hello, World!".to_owned()
    });

    assert_eq!(value, "Hello, World!");

    // Subsequent calls to `get_or_init` will not re-initialize,
    // but return the already initialized value.
    let same_value = cell.get_or_init(|| {
        println!("This will not be printed");
        "Goodbye!".to_owned()
    });

    assert_eq!(same_value, "Hello, World!");

    // You can also use `set`, which returns an error if already initialized
    let result = cell.set("New Value".to_owned());
    assert!(result.is_err());

    // `get` returns an Option with a reference to the value
    assert_eq!(cell.get().unwrap(), "Hello, World!");
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
