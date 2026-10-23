#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Creating and unwrapping `Option` values.

/// Creates `Some` and `None` values and unwraps them.
fn main() {
    // `Some` is a variant of the `Option` enum that represents a value.
    let some_number = Some(5);
    println!("{some_number:?}");

    // `None` is another variant of the `Option` enum
    // that represents the _absence_ of a value.
    let absent_number: Option<i32> = None;
    println!("{absent_number:?}");
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
