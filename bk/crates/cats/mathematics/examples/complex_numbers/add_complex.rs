#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Demonstrates the manipulation of complex numbers.

/// Adds two complex numbers and prints the sum.
fn main() {
    // Create two complex numbers.
    let complex_num1 = num::complex::Complex::new(10.0, 20.0);
    let complex_num2 = num::complex::Complex::new(3.1, -4.2);

    // Add the two complex numbers.
    let sum = complex_num1 + complex_num2;

    println!("Sum: {sum}");
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the addition example runs without panicking.
    #[test]
    fn test_main() {
        main();
    }
}
