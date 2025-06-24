#![allow(dead_code)]
// ANCHOR: example
fn main() {
    let number = 3;
    // The condition must be a boolean.
    if number > 0 {
        // If a condition operand evaluates to true,
        // the consequent block is executed and any subsequent else if or else
        // block is skipped.
        println!("Positive number");
    } else if number < 0 {
        println!("Negative number");
    } else {
        println!("Zero");
    }

    // Note that `if` is an expression, meaning it evaluates to a value.
    let result: u8 = if number < 5 {
        println!("Condition was true");
        5
    } else {
        println!("Condition was false");
        // The value of the `if` expression is the value of the last expression
        // in the executed block.
        6
    }; // Semicolon here, because `let` is a statement.
    println!("{}", result);
}
// All branches of an `if/else` expression must evaluate to the same type.
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
