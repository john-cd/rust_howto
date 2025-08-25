#![allow(dead_code)]
// ANCHOR: example
#![allow(unused_assignments)]
//! This example demonstrates common string types in Rust.

// `String` is a growable, heap-allocated data structure that is
// Unicode, not ASCII.
fn string() {
    // Create an empty string:
    let mut s1 = String::new();

    // Create a `String` from a string literal via the `From` trait.
    s1 = String::from("hello");

    // `String` can be mutated:
    s1.push(',');
    // `push_str` appends a string slice to a `String`.
    s1.push_str(" world!");
    // `clear` empties the String, making it equal to "".
    s1.clear();

    // Alternative initialization from string literals:
    // `to_string` is available on any type that implements
    // the `Display` trait.
    s1 = "contents".to_string();

    println!("String: {s1}");
}

// The `str` type, also called a 'string slice', is the most primitive string
// type. It is usually seen in its borrowed form, `&str`.
fn string_slices() {
    let my_string = String::from("Rust is awesome!");

    // Create a string slice pointing to the entire `String`:
    let slice1 = &my_string[..]; // Using `Index<RangeFull, Output = str>`.
    println!("Slice of an entire `String`: '{slice1}'");

    // The following works as well:
    let _slice2 = &*my_string; // Using `Deref<Target = str>`.
    let _slice3: &str = &my_string; // Type ascription.

    // Create a string slice pointing to a part of the `String`:
    let slice4: &str = &my_string[0..4]; // "Rust"
    println!("Slice of part of a `String`: '{slice4}'");
    // Caution: If you were to try to slice only part of a unicode
    // character's bytes, Rust would panic at runtime.
    // String slices must always be valid UTF-8!

    // Convert a `String` to `&str` explicitly using `as_str()`.
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
    let string_from_literal: String = literal.to_string();
    println!("String from literal: '{string_from_literal}'");
}

fn common_operations() {
    let s1 = String::from("hello");
    let s2 = String::from(" Rust");

    // Concatenation:
    let _s = s1 + &s2;
    // Note: s1 is moved and can no longer be used afterwards.
    // The `+` operator takes ownership of the first string and appends
    // a string slice to it.

    // Formatting:
    let s1 = String::from("hello");
    let _s = format!("{s1}{s2}");

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

fn main() {
    string();
    string_slices();
    string_literals();
    common_operations();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
