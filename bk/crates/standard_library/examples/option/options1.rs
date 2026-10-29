// TODO Reviewed ??-??-2026.

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

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options1_example_entry_point() {
        main();
    }

    #[test]
    fn test_some_and_none() {
        let some_number = Some(5);
        let absent_number: Option<i32> = None;
        assert!(some_number.is_some());
        assert!(absent_number.is_none());
        assert_eq!(format!("{some_number:?}"), "Some(5)");
        assert_eq!(format!("{absent_number:?}"), "None");
    }
}
