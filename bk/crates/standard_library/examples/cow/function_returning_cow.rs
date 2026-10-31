// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Example of a function returning a `Cow`.
//!
//! Returning `Cow` lets a function return its input untouched (no allocation)
//! when nothing needs changing, and a new `String` only when it does, as in
//! escaping or normalizing text.

use std::borrow::Cow;

/// This function takes a string slice and returns a `Cow<str>`.
/// If the input string contains spaces, it returns an owned `String` with
/// spaces removed. Otherwise, it returns a borrowed `&str` pointing to the
/// original string slice. This avoids unnecessary allocation when no spaces are
/// present.
fn remove_whitespaces(s: &str) -> Cow<'_, str> {
    if s.contains(' ') {
        // Allocate, since we need to modify the string.
        Cow::Owned(s.to_owned().replace(' ', ""))
    } else {
        // Nothing to do: just return the string slice.
        Cow::Borrowed(s)
    }
}

/// Removes the spaces of strings, allocating only when needed.
fn main() {
    let original_string = "Hello world!  ";
    let modified_string = remove_whitespaces(original_string);

    match modified_string {
        Cow::Borrowed(s) => println!("No spaces, no allocation: {s}"),
        Cow::Owned(s) => println!("Spaces removed, allocated `String`: {s}"),
    }

    let original_string_no_spaces = "HelloWorld!";
    let modified_string_no_spaces =
        remove_whitespaces(original_string_no_spaces);

    match modified_string_no_spaces {
        Cow::Borrowed(s) => println!("No spaces, no allocation: {s}"),
        Cow::Owned(s) => println!("Spaces removed, allocated String: {s}"),
    }
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_returning_cow_example_entry_point() {
        main();
    }

    #[test]
    fn test_spaces_are_removed_with_an_allocation() {
        let result = remove_whitespaces("Hello world!  ");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "Helloworld!");
    }

    #[test]
    fn test_no_spaces_means_no_allocation() {
        let input = "HelloWorld!";
        let result = remove_whitespaces(input);
        assert!(matches!(result, Cow::Borrowed(_)));
        // The result points into the input.
        assert!(std::ptr::eq(result.as_ref(), input));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(remove_whitespaces(""), "");
        assert!(matches!(remove_whitespaces(""), Cow::Borrowed(_)));
        assert_eq!(remove_whitespaces("   "), "");
        // Only the ASCII space is removed.
        assert_eq!(remove_whitespaces("a\tb"), "a\tb");
    }
}
