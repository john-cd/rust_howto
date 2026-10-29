// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Modifying a `Cow` in place, cloning it on write.

use std::borrow::Cow;

/// Demonstrates modifying a `Cow` in place.
///
/// If the input `Cow` contains "Rust", it appends " is awesome!" to it.
/// If the `Cow` is borrowed, it will be cloned into an owned `String` before
/// modification.
fn modify_string(input: &mut Cow<str>) {
    // Note the mutable reference.
    if input.contains("Rust") {
        // `to_mut()` will clone the `Cow` if it's borrowed,
        // ensuring we have an owned `String` to modify.
        input.to_mut().push_str(" is awesome!");
    }
}

/// Modifies a borrowed and an owned `Cow`.
fn main() {
    let mut borrowed = Cow::Borrowed("Hello");
    modify_string(&mut borrowed);
    println!("{borrowed}");

    let mut borrowed_to_owned = Cow::Borrowed("Rust");
    modify_string(&mut borrowed_to_owned);
    println!("{borrowed_to_owned}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modify_cow_in_place_example_entry_point() {
        main();
    }

    #[test]
    fn test_unmatched_input_stays_borrowed() {
        let mut c = Cow::Borrowed("Hello");
        modify_string(&mut c);
        assert!(matches!(c, Cow::Borrowed("Hello")));
    }

    #[test]
    fn test_matching_input_is_cloned_on_write() {
        let mut c = Cow::Borrowed("Rust");
        modify_string(&mut c);
        assert!(matches!(c, Cow::Owned(_)));
        assert_eq!(c, "Rust is awesome!");
    }

    #[test]
    fn test_already_owned_input_is_modified_in_place() {
        let mut c: Cow<str> = Cow::Owned("Rust".to_owned());
        modify_string(&mut c);
        assert_eq!(c, "Rust is awesome!");
    }
}
