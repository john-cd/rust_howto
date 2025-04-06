// ANCHOR: example

use std::str::FromStr;

use num_bigint::BigInt;
use num_traits::One;

/// Demonstrates the usage of `BigInt` for large integer arithmetic.
fn main() {
    // Parse big integers from strings.
    let a = BigInt::from_str("123456789012345678901234567890").unwrap();
    let b = BigInt::from_str("987654321098765432109876543210").unwrap();

    // Add.
    let sum = &a + &b;
    println!("Sum: {}", sum);

    // Subtract.
    let difference = &b - &a;
    println!("Difference: {}", difference);

    // Multiply.
    let product = &a * &b;
    println!("Product: {}", product);

    // Divide.
    let quotient = &b / &a;
    println!("Quotient: {}", quotient);

    // Create a big integer with value 1.
    let one = BigInt::one();
    println!("One: {}", one);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
