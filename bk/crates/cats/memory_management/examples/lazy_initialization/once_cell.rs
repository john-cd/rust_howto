// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Lazy initialization of a value with `OnceCell`.
//!
//! A `OnceCell` starts empty and can be set exactly once; later reads return
//! the stored value. It is for single-threaded code, and the thread-safe
//! version is `OnceLock`.

use std::cell::OnceCell;

/// Initializes a `OnceCell` on first access.
fn main() {
    // Create a new OnceCell instance.
    let cell = OnceCell::new();
    // The cell is initially empty.
    assert!(cell.get().is_none());

    let value: &String = cell.get_or_init(|| "Hello, World!".to_owned());
    println!("{value}");
    assert_eq!(value, "Hello, World!");
    assert!(cell.get().is_some());
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the once cell example through its entry point.
    #[test]
    fn test_once_cell_example_entry_point() {
        main();
    }

    /// `get_or_init` keeps the first value and ignores later initializers.
    #[test]
    fn first_initialization_wins() {
        let cell = OnceCell::new();
        assert_eq!(cell.get_or_init(|| 1), &1);
        assert_eq!(cell.get_or_init(|| 2), &1);
        assert_eq!(cell.set(3), Err(3));
    }
}
