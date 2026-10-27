#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Example of a function accepting either owned or borrowed values
//! at its input.

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

    /// Runs the cow as function param example through its entry point.
    #[test]
    fn test_cow_as_function_param_example_entry_point() {
        main();
    }
}
