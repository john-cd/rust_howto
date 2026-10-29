// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! String slice examples.

/// Creates string slices from a string literal and a `String`.
fn main() {
    // A string literal:
    let string_literal: &'static str = "hello world";

    // An owned `String` created from the literal:
    let s: String = String::from(string_literal);

    // String slices from the `String`:
    let hello: &str = &s[0..5]; // or &s[..5];
    let world: &str = &s[6..11]; // or &s[6..];
    println!("{hello}");
    println!("{world}");

    // Take a slice of the entire `String`:
    let whole_slice = &*s;

    // A string slice is a view into the `String`:
    // The inner pointers of the slice and the string are equal.
    assert!(std::ptr::eq(whole_slice.as_ptr(), s.as_ptr()));
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the string slices example through its entry point.
    #[test]
    fn test_string_slices_example_entry_point() {
        main();
    }

    /// The slices cover the expected parts of the string.
    #[test]
    fn test_string_slices() {
        let s = String::from("hello world");
        assert_eq!(&s[0..5], "hello");
        assert_eq!(&s[6..], "world");
        assert_eq!(&s[..5], &s[0..5]);
    }

    /// Slicing in the middle of a multi-byte character panics.
    #[test]
    #[should_panic(expected = "is not a char boundary")]
    fn test_slice_not_on_char_boundary() {
        let s = String::from("héllo");
        let end = 2;
        let _ = &s[0..end];
    }
}
