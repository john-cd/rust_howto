// ANCHOR: example
//! Example of a function returning a `Cow`.

use std::borrow::Cow;

/// This function takes a string slice and returns a `Cow<str>`.
/// If the input string contains spaces, it returns an owned `String` with
/// spaces removed. Otherwise, it returns a borrowed `&str` pointing to the
/// original string slice. This avoids unnecessary allocation when no spaces are
/// present.
fn remove_whitespaces(s: &str) -> Cow<str> {
    if s.contains(' ') {
        // Allocate, since we need to modify the string.
        Cow::Owned(s.to_string().replace(' ', ""))
    } else {
        // Nothing to do: just return the string slice.
        Cow::Borrowed(s)
    }
}

fn main() {
    let original_string = "Hello world!  ";
    let modified_string = remove_whitespaces(original_string);

    match modified_string {
        Cow::Borrowed(s) => println!("No spaces, no allocation: {}", s),
        Cow::Owned(s) => println!("Spaces removed, allocated `String`: {}", s),
    }

    let original_string_no_spaces = "HelloWorld!";
    let modified_string_no_spaces =
        remove_whitespaces(original_string_no_spaces);

    match modified_string_no_spaces {
        Cow::Borrowed(s) => println!("No spaces, no allocation: {}", s),
        Cow::Owned(s) => println!("Spaces removed, allocated String: {}", s),
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
