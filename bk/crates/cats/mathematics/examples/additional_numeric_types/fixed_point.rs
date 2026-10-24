#![allow(dead_code)]
// ANCHOR: example
//! Divides with a fixed-point type whose fractional precision is set in its
//! type.

use fixed::types::I20F12;

/// Divides an integer by three using twelve fractional bits.
fn main() {
    let quotient = I20F12::from_num(19) / 3;

    println!("19 / 3 = {quotient}");
    assert_eq!(quotient.to_string(), "6.3333");
}

// ANCHOR_END: example

/// Runs the fixed-point example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies fixed-point division produces the expected rounded decimal
    /// display.
    #[test]
    fn divides_with_fixed_fractional_precision() {
        main();
    }
}
