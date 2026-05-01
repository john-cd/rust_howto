#![allow(dead_code)]
// ANCHOR: example
/// The `#[must_use]` attribute indicates that the return value of a function
/// must be used. This attribute can also be applied to traits, structs, enums,
/// etc.
#[must_use]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    // Use the return value of the function.
    println!("{}", add(1, 2));

    // Uncomment the following to see the warning:
    // "unused return value of `attributes_must_use::add` that must be used"
    // WARNING: add(2, 3);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
