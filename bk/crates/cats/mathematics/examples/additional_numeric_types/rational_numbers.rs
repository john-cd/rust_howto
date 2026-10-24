#![allow(dead_code)]
// ANCHOR: example
//! Adds rational numbers without converting them to floating-point values.

use num_rational::Ratio;

/// Adds one third and one sixth exactly.
fn main() {
    let third = Ratio::new(1_i64, 3);
    let sixth = Ratio::new(1_i64, 6);
    let half = third + sixth;

    println!("1/3 + 1/6 = {half}");
    assert_eq!(half, Ratio::new(1_i64, 2));
}

// ANCHOR_END: example

/// Runs the rational-number example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that rational addition preserves the exact result.
    #[test]
    fn adds_rational_values_exactly() {
        main();
    }
}
