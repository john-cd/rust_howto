#![allow(dead_code)]
// ANCHOR: example
/// This is an example of a `while` loop.
fn main() {
    // Initialize a mutable variable.
    let mut number = 5;
    // The loop continues as long as the condition is true.
    while number != 0 {
        print!("{number} ");
        // Decrement the number.
        number -= 1;
    }
    // Prints: 5 4 3 2 1
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
