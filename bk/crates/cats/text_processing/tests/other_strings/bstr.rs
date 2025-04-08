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

fn main() {
    // Basic usage:
    let _bstring = BString::from("Hello, world!");
    let bytes: Vec<u8> = vec![72, 101, 108, 108, 111]; // "Hello"
    let _bstring_from_bytes = BString::from(bytes);

    // Working with non-UTF8 data:
    let invalid_utf8 = vec![72, 101, 108, 108, 111, 0xFF, 0xFE, 33];
    let bstring_invalid = BString::from(invalid_utf8);
    println!("BString with invalid UTF-8: {}", bstring_invalid);

    // `ByteSlice` methods:
    let text: &BStr = b"apple,banana,cherry".as_bstr();
    for item in text.split_str(",") {
        println!("Item: {:?}", item);
    }

    // Find substrings:
    let haystack = b"Finding a needle in a haystack".as_bstr();
    if let Some(pos) = haystack.find("needle") {
        println!("Found 'needle' at position: {}", pos);
    }

    // Modifying a `BString`:
    let mut growable = BString::from("Growing ");
    growable.push_str("string");

    // Line handling:
    let multiline = b"First line\nSecond line\r\n".as_bstr();
    for line in multiline.lines() {
        println!("Line: {:?}", line);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
