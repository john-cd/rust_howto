// ANCHOR: example
#![allow(clippy::unnecessary_literal_unwrap)]
//! Demonstrates the use of `unwrap()` and `expect()` for handling `Result`
//! types.

fn main() {
    let number_str = "42";

    // `parse()` attempts to convert the string into a number.
    // This operation can fail if the string isn't a valid number, so it returns
    // a `Result<u32, ParseIntError>`.
    let result: Result<u32, std::num::ParseIntError> = number_str.parse();

    // If the error is not recoverable at this point, we can cause the
    // program to crash if this instance of `Result` is an `Err` value:

    // 1. `unwrap()` can be called on the `Result` to extract the `u32` value.
    // - If the parsing is successful, the value is assigned to `number`.
    // - If the parsing fails, the program panics with an error message.
    let number: u32 = result.unwrap();

    println!("The number is: {}", number);

    // 2. `expect` is similar (and often preferred) to `unwrap`.
    // If `Result` is an `Err` value, it panics and displays the message
    // passed as an argument.
    let result: Result<i32, &str> = Ok(10);
    result.expect("Failed to do something.");
    // You may also use `unwrap`, which panics if there is an error but does not
    // display a custom message: `result.unwrap();`
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
