#![allow(dead_code)]
// ANCHOR: example

// Tuple struct
struct Color(i32, i32, i32);

// Unit-like struct
struct AlwaysEqual; // <-- no fields

fn main() {
    let _black = Color(0, 0, 0);
    let _s = AlwaysEqual;
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// TODO
