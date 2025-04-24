// ANCHOR: example
/// The `#[must_use]` attribute indicates that the results of a function must be
/// used. This attribute can also be applied to traits, structs, enums, etc.
#[must_use]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("{}", add(1, 2));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
