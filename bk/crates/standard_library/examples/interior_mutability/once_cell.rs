// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! `OnceCell<T>` allows for single assignment, interior mutability.
//! It's typically used for lazy initialization of a value.
//!
//! Interior mutability changes data behind a shared reference (`&T`), moving
//! the borrow check from compile time to run time. `OnceCell` can be set once,
//! and later reads are free.

use std::cell::OnceCell;

/// Initializes a `OnceCell` once and reads it.
fn main() {
    let cell = OnceCell::new();

    // The cell is initially empty.
    assert!(cell.get().is_none());

    // Initialize the cell once.
    let value = cell.get_or_init(|| {
        println!("Initializing...");
        "Hello, World!".to_owned()
    });

    assert_eq!(value, "Hello, World!");

    // Subsequent calls to `get_or_init` return the stored value without
    // running the initializer again.
    let same_value = cell.get_or_init(|| {
        println!("This will not be printed");
        "Goodbye!".to_owned()
    });

    assert_eq!(same_value, "Hello, World!");

    // You can also call `set`, which returns an error if the cell is already
    // initialized.
    let result = cell.set("New Value".to_owned());
    assert!(result.is_err());

    // `get` returns an `Option` containing a reference to the stored value.
    assert_eq!(
        cell.get()
            .expect("expected example value should be present"),
        "Hello, World!"
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_once_cell_example_entry_point() {
        main();
    }

    #[test]
    fn test_get_or_init_runs_the_closure_once() {
        let cell = OnceCell::new();
        let mut calls = 0;
        assert_eq!(cell.get(), None);
        assert_eq!(
            *cell.get_or_init(|| {
                calls += 1;
                1
            }),
            1
        );
        assert_eq!(
            *cell.get_or_init(|| {
                calls += 1;
                2
            }),
            1
        );
        assert_eq!(calls, 1);
    }

    #[test]
    fn test_set_fails_once_initialized() {
        let cell = OnceCell::new();
        assert_eq!(cell.set(1), Ok(()));
        assert_eq!(cell.set(2), Err(2));
        assert_eq!(cell.get(), Some(&1));
        assert_eq!(cell.into_inner(), Some(1));
    }
}
