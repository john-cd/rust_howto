#![allow(dead_code)]
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
    println!("Sum: {sum}");
    assert_eq!(sum.to_string(), "1111111110111111111011111111100");

    // Subtract.
    let difference = &b - &a;
    println!("Difference: {difference}");
    assert_eq!(difference.to_string(), "864197532086419753208641975320");

    // Multiply.
    let product = &a * &b;
    println!("Product: {product}");
    assert_eq!(
        product.to_string(),
        "121932631137021795226185032733622923332237463801111263526900"
    );

    // Divide.
    let quotient = &b / &a;
    println!("Quotient: {quotient}");
    assert_eq!(quotient.to_string(), "8");

    // Create a big integer with value 1.
    let one = BigInt::one();
    println!("One: {one}");
    assert_eq!(one.to_string(), "1");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        main();
    }
}
