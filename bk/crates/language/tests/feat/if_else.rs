// ANCHOR: example
fn main() {
    let number = 3;
    // Note that `if` is an expression.
    let result: u8 = if number < 5 {
        println!("Condition was true");
        // The value of the `if` expression is the value of the last expression
        // in the block.
        5
    } else {
        println!("Condition was false");
        6
    };
    println!("{}", result);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
