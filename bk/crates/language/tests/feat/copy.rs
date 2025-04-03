// ANCHOR: example

/// Integers implement the `Copy` trait, so they are copied instead of moved.
fn main() {
    let x = 5; // x is an integer.
    let y = x; // y is a copy of x.

    // Both x and y are valid here.
    println!("x = {}, y = {}", x, y);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
