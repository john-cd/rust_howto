#![allow(dead_code)]
// ANCHOR: example
/// A tuple struct.
///
/// Note the ( ) and the lack of field names.
#[derive(Debug)]
struct Color(i32, i32, i32);

/// A unit-like struct.
///
/// Unit-like structs are useful when you need to implement a trait on
/// something, but don't have any data that you want to store in the type
/// itself.
#[derive(Debug)]
struct AlwaysEqual; // Note that there are no fields.

fn main() {
    let black = Color(0, 0, 0);
    println!("{black:?}");

    let s = AlwaysEqual;
    println!("{s:?}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
