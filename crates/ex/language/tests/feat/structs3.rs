#![allow(dead_code)]
// ANCHOR: example

// A tuple struct
// Note the ( ) and the lack of field names.
#[derive(Debug)]
struct Color(i32, i32, i32);

// A unit-like struct
#[derive(Debug)]
struct AlwaysEqual; // Note that there are no fields

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
// [P1](https://github.com/john-cd/rust_howto/issues/876)
// [structs3: example (P1)](https://github.com/john-cd/rust_howto/issues/182)
