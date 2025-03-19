#![allow(unused)]
// ANCHOR: example
fn main() {
    // `const` is set to a constant expression
    // The type must be annotated
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // ERROR: THREE_HOURS_IN_SECONDS = 10800;

    // Immutable variable
    let apples = 5;
    // ERROR: apples = 6;
    println!("apples: {}", apples);

    // Mutable variable
    let mut guess = String::new();
    guess.push_str("42");
    println!("guess: {}", guess);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
