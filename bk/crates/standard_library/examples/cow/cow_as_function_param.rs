// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Example of a function accepting either owned or borrowed values
//! at its input.
//!
//! Accepting `Cow` lets a caller pass either a borrowed `&str` or an owned
//! `String`, and the function allocates only if it must change the text.

use std::borrow::Cow;

/// A function consuming a `Cow`:
fn process_text(input: Cow<str>) {
    if input.contains("Rust") {
        println!("Contains Rust!");
    }
}

/// Calls a function with borrowed and owned strings.
fn main() {
    // Call the function with a `&str`, here a string literal.
    let borrowed_text: Cow<str> = Cow::Borrowed("Hello, Rust!");
    process_text(borrowed_text);

    // Call the function with an owned `String`.
    let owned_text: Cow<str> = Cow::Owned(String::from("Hello, world!"));
    process_text(owned_text);

    // We can also pass a `String` or string slice directly, calling `into()`.
    let direct_string = String::from("Another example");
    process_text(direct_string.into());
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cow_as_function_param_example_entry_point() {
        main();
    }

    #[test]
    fn test_process_text_accepts_every_kind_of_input() {
        // Each of these compiles because `&str` and `String` convert to `Cow`.
        process_text(Cow::Borrowed("Hello, Rust!"));
        process_text(Cow::Owned(String::from("Hello, world!")));
        process_text("literal".into());
        process_text(String::from("owned").into());
    }

    #[test]
    fn test_conversions_pick_the_right_variant() {
        assert!(matches!(Cow::from("a"), Cow::Borrowed("a")));
        assert!(matches!(Cow::from(String::from("a")), Cow::Owned(_)));
    }
}
