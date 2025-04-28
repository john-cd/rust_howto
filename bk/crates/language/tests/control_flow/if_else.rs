// ANCHOR: example
fn main() {
    let number = 3;
    // Note that `if` is an expression, meaning it evaluates to a value.
    let result: u8 = if number < 5 {
        println!("Condition was true");
        // The value of the `if` expression is the value of the last expression
        // in the block.
        5
    } else {
        println!("Condition was false");
        6
    }; // Semicolon here, because `let` is a statement.
    println!("{}", result);
}
// All branches of an `if/else` expression must evaluate to the same type,
// if you intend to assign the result to a variable.
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
