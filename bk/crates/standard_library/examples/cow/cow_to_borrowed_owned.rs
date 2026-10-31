// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates various uses and conversions of `Cow` (Clone-on-Write).
//!
//! `Cow` (clone on write) holds either a borrowed or an owned value and clones
//! only when needed. `into_owned` gives a fully owned value; `to_mut` clones if
//! the data is still borrowed.

use std::borrow::Borrow as _;
use std::borrow::Cow;

/// Demonstrates various uses and conversions of `Cow` (Clone-on-Write).
fn main() {
    let mut my_string = String::new();

    // Create a `Cow` from the string literal "Example ".
    let example: Cow<str> = Cow::from("Example ");

    // Append the `Cow` to a `String` (conversion to a `&str`).
    my_string.push_str(&example);
    println!("{my_string}");
    // Alternatively, use `borrow`, `as_ref` or deref coercion:
    my_string.push_str(example.borrow());
    println!("{my_string}");
    my_string.push_str(example.as_ref());
    println!("{my_string}");
    my_string.push_str(&example);
    println!("{my_string}");
    // Unstable feature: my_string.push_str(&example.as_str());

    // Convert a `Cow<str>` to an owned `String`:
    let s: String = example.to_string();
    println!("{s}");
    // We could also convert the `Cow` to a borrowed reference using
    // `as_ref()`, then clone the borrowed reference to create an owned
    // `String`.
    println!("{}", example.as_ref().to_owned());
    // `into_owned` extracts the owned data and clones the data if it is not
    // already owned.
    println!("{}", example.into_owned());
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cow_to_borrowed_owned_example_entry_point() {
        main();
    }

    #[test]
    fn test_borrowing_from_a_cow() {
        let example: Cow<str> = Cow::from("Example ");
        let mut s = String::new();
        s.push_str(&example);
        s.push_str(example.borrow());
        s.push_str(example.as_ref());
        assert_eq!(s, "Example Example Example ");
        // The `Cow` is untouched, and still borrowed.
        assert!(matches!(example, Cow::Borrowed(_)));
    }

    #[test]
    fn test_converting_to_owned() {
        let example: Cow<str> = Cow::from("Example ");
        assert_eq!(example.to_string(), "Example ");
        assert_eq!(example.as_ref().to_owned(), "Example ");
        let owned: String = example.into_owned();
        assert_eq!(owned, "Example ");
    }
}
