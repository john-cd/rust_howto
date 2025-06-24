#![allow(dead_code)]
// ANCHOR: example
use std::borrow::Cow;

/// Demonstrates modifying a `Cow` in place.
///
/// If the input `Cow` contains "Rust", it appends " is awesome!" to it.
/// If the `Cow` is borrowed, it will be cloned into an owned `String` before
/// modification.
fn modify_string(input: &mut Cow<str>) {
    // Note the mutable reference.
    if input.contains("Rust") {
        // `to_mut()` will clone the `Cow` if it's borrowed,
        // ensuring we have an owned `String` to modify.
        input.to_mut().push_str(" is awesome!");
    }
}

fn main() {
    let mut borrowed = Cow::Borrowed("Hello");
    modify_string(&mut borrowed);
    println!("{}", borrowed);

    let mut borrowed_to_owned = Cow::Borrowed("Rust");
    modify_string(&mut borrowed_to_owned);
    println!("{}", borrowed_to_owned);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
