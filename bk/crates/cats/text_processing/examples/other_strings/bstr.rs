// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! The `bstr` crate provides types and traits for working with byte strings,
//! which are sequences of bytes that may or may not be valid UTF-8.
//!
//! Add to your `Cargo.toml` file:
//! ```toml
//! [dependencies]
//! bstr = "1.11.3" # Or latest
//! ```

// BStr is a byte string slice, analogous to str. It represents a borrowed
// sequence of bytes that may or may not be valid UTF-8.
use bstr::BStr;
// BString is an owned growable byte string buffer, analogous to String. It
// represents an owned sequence of bytes that may or may not be valid
// UTF-8.
use bstr::BString;
// ByteSlice extends the `[u8]` type with additional string-oriented
// methods. This trait is implemented for `[u8]` and `&[u8]`, providing
// methods for searching, splitting, and other operations on byte slices.
use bstr::ByteSlice;
// ByteVec extends the `Vec<u8>` type with additional string-oriented
// methods. This trait is implemented for `Vec<u8>`, providing methods for
// pushing, extending, and other operations on byte vectors.
use bstr::ByteVec;

/// Creates and manipulates byte strings, which may not be valid UTF-8.
fn main() {
    // Basic usage:
    let _bstring = BString::from("Hello, world!");
    let bytes: Vec<u8> = vec![72, 101, 108, 108, 111]; // "Hello"
    let _bstring_from_bytes = BString::from(bytes);

    // Working with non-UTF8 data:
    let invalid_utf8 = vec![72, 101, 108, 108, 111, 0xFF, 0xFE, 33];
    let bstring_invalid = BString::from(invalid_utf8);
    println!("BString with invalid UTF-8: {bstring_invalid}");

    // `ByteSlice` methods:
    let text: &BStr = b"apple,banana,cherry".as_bstr();
    for item in text.split_str(",") {
        println!("Item: {item:?}");
    }

    // Find substrings:
    let haystack = b"Finding a needle in a haystack".as_bstr();
    if let Some(pos) = haystack.find("needle") {
        println!("Found 'needle' at position: {pos}");
    }

    // Modifying a `BString`:
    let mut growable = BString::from("Growing ");
    growable.push_str("string");

    // Line handling:
    let multiline = b"First line\nSecond line\r\n".as_bstr();
    for line in multiline.lines() {
        println!("Line: {line:?}");
    }
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the bstr example through its entry point.
    #[test]
    fn test_bstr_example_entry_point() {
        main();
    }

    /// Invalid UTF-8 is kept in the byte string and shown lossily.
    #[test]
    fn keeps_invalid_utf8() {
        let s = BString::from(vec![72, 101, 0xFF, 33]);
        assert_eq!(s.len(), 4);
        assert!(std::str::from_utf8(s.as_bytes()).is_err());
        assert_eq!(s.to_str_lossy(), "He\u{FFFD}!");
    }

    /// Splitting, searching and line iteration work on bytes.
    #[test]
    fn splits_finds_and_iterates_lines() {
        let text: &BStr = b"apple,banana,cherry".as_bstr();
        let items: Vec<_> = text.split_str(",").collect();
        assert_eq!(
            items,
            [b"apple".as_bstr(), b"banana".as_bstr(), b"cherry".as_bstr()]
        );
        assert_eq!(b"a needle here".as_bstr().find("needle"), Some(2));
        assert_eq!(b"a haystack".as_bstr().find("needle"), None);
        let lines: Vec<_> = b"one\ntwo\r\n".as_bstr().lines().collect();
        assert_eq!(lines, [b"one".as_bstr(), b"two".as_bstr()]);
    }

    /// Strings can be appended to.
    #[test]
    fn push_str_appends() {
        let mut s = BString::from("Growing ");
        s.push_str("string");
        assert_eq!(s, "Growing string");
    }
}
