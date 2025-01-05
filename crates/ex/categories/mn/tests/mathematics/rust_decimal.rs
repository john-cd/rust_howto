// ANCHOR: example
use std::str::FromStr;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

// Handles decimal numbers with high precision, which can be particularly useful
// for financial calculations.
fn main() {
    // Creating decimals from literals
    let number1 = Decimal::new(1234, 2); // 12.34
    let number2 = dec!(56.78);

    // Arithmetic operations
    let sum = number1 + number2;
    let difference = number1 - number2;
    let product = number1 * number2;
    let quotient = number1 / number2;

    // Printing the results
    println!("Number 1: {}", number1);
    println!("Number 2: {}", number2);
    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);

    // Converting to and from strings
    let number_str = "98.76";
    let number_from_str = Decimal::from_str(number_str).unwrap();
    println!("Number from string: {}", number_from_str);

    let number_to_str = number_from_str.to_string();
    println!("Number to string: {}", number_to_str);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
