// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates common string types in Rust.
//!
//! Rust has two main string types: `String`, an owned, growable UTF-8 buffer,
//! and `&str`, a borrowed view into one. Indexing by byte position can split a
//! multi-byte character, so Rust makes you ask for characters or bytes
//! explicitly.

// `String` stores UTF-8 bytes and can represent Unicode text; ASCII is a subset
// of UTF-8.
/// Creates and mutates a `String`.
fn string() {
    // Create an empty string:
    let mut s1 = String::new();
    assert!(s1.is_empty());

    // Create a `String` from a string literal via the `From` trait.
    s1 = String::from("hello");

    // `String` can be mutated:
    s1.push(',');
    // `push_str` appends a string slice to a `String`.
    s1.push_str(" world!");
    // `clear` empties the String, making it equal to "".
    s1.clear();

    // Alternative initialization from string literals:
    // `to_owned()` creates an owned `String` from a string slice.
    s1 = "contents".to_owned();
    assert_eq!(s1, "contents");

    println!("String: {s1}");
}

// The `str` type, also called a 'string slice', is the most primitive string
// type. It is usually seen in its borrowed form, `&str`.
/// Creates string slices from a `String`.
#[expect(
    clippy::deref_by_slicing,
    reason = "This example demonstrates indexing a String with a full range."
)]
fn string_slices() {
    let my_string = String::from("Rust is awesome!");

    // Create a string slice pointing to the entire `String`:
    let slice1 = &my_string[..]; // Using `Index<RangeFull, Output = str>`.
    println!("Slice of an entire `String`: '{slice1}'");

    // The following works as well:
    let _slice2 = &*my_string; // Using `Deref<Target = str>`.
    let _slice3: &str = &my_string; // Type annotation; deref coercion converts `&String` to `&str`.

    // Create a string slice pointing to a part of the `String`:
    let slice4: &str = &my_string[0..4]; // "Rust"
    println!("Slice of part of a `String`: '{slice4}'");
    assert_eq!(slice4, "Rust");
    // Caution: If you were to try to slice only part of a unicode
    // character's bytes, Rust would panic at runtime.
    // String slices must always be valid UTF-8!

    // Convert a `String` to `&str` explicitly using `as_str()`.
    /// Prints the contents of the supplied slice.
    fn print_slice(s: &str) {
        println!("Printing a slice: '{s}'");
    }
    print_slice(my_string.as_str());

    // You can also convert bytes (in a vector or an array) into a string slice
    // via `std::str::from_utf8`, _if the bytes are valid UTF-8_.
}

/// String literals are immutable, fixed-size sequences of characters that are
/// known at compile time. They are typically embedded directly into your
/// program's binary.
fn string_literals() {
    // String literals are string slices of type `&'static str`.
    let literal: &'static str = "Hello, world!";
    println!("String literal: '{literal}'");

    // We can't directly modify a string literal.
    // ERROR: literal.push_str("!");

    // String literals can be used to create a `String`.
    let string_from_literal: String = literal.to_owned();
    println!("String from literal: '{string_from_literal}'");
}

/// Concatenates, formats and iterates over strings.
fn common_operations() {
    let s1 = String::from("hello");
    let s2 = String::from(" Rust");

    // Concatenation:
    let _s = s1 + &s2;
    assert_eq!(_s, "hello Rust");
    // Note: s1 is moved and can no longer be used afterwards.
    // The `+` operator takes ownership of the first string and appends
    // a string slice to it.

    // Formatting:
    let s1 = String::from("hello");
    let _s = format!("{s1}{s2}");
    assert_eq!(_s, "hello Rust");

    // Iteration:
    // Character by character:
    for c in "Зд".chars() {
        println!("{c}");
    }
    // Byte by byte:
    for b in "Зд".bytes() {
        println!("{b}");
    }
}

/// Runs all the string examples.
fn main() {
    string();
    string_slices();
    string_literals();
    common_operations();
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the strings example through its entry point.
    #[test]
    fn test_strings_example_entry_point() {
        main();
    }
}
