#![allow(unused)]
// ANCHOR: example
/// Variables and constants are used to store values.
/// Variables can be mutable or immutable.
/// Constants are always immutable.
fn main() {
    // Constants are always immutable.
    // The type must be provided.
    // Here, `const` is set to a constant expression.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Immutable variable:
    let apples = 5;
    // You cannot reassign an immutable variable.
    // ERROR: apples = 6;
    println!("apples: {}", apples);

    // Mutable variable - note the `mut` keyword.
    let mut guess = String::new();
    // You can modify a mutable variable.
    guess.push_str("42");
    println!("guess: {}", guess);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
