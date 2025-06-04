#![allow(dead_code)]
// ANCHOR: example
/// A tuple struct.
///
/// Note the ( ) and the lack of field names.
/// It behaves like a named tuple.
#[derive(Debug)]
struct Color(i32, i32, i32);

/// A unit-like struct.
///
/// Unit-like structs are useful when you need to implement a trait on
/// something, but don't have any data that you want to store in the type
/// itself.
#[derive(Debug)]
struct UnitLike; // Note that there are no fields.

fn main() {
    // Initialize the tuple struct:
    let black = Color(0, 0, 0);
    // Access the inner field with .0, .1, etc.:
    let _red_value = black.0;
    println!("{black:?}");

    let s = UnitLike;
    println!("{s:?}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
