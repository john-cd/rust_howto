#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates big integer manipulation.

use num::bigint::BigInt;
use num::bigint::ToBigInt;

/// Calculates the factorial of a non-negative integer using `BigInt`.
fn factorial(x: u32) -> BigInt {
    if let Some(mut factorial) = 1.to_bigint() {
        for i in 1..=x {
            factorial *= i;
        }
        factorial
    } else {
        panic!("Failed to calculate factorial!");
    }
}

fn main() {
    println!("100! equals {}", factorial(100));
    assert_eq!(factorial(100).to_string().len(), 158); // factorial(100) is a 158-digit number
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
