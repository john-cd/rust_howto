#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Compare floating-point values with `float-cmp` and `approx`.
//!
//! `0.1 + 0.2` is not exactly `0.3` in binary floating point, so exact
//! equality is usually the wrong assertion for calculated values.

use approx::assert_relative_eq;

/// Demonstrates ULP and epsilon comparisons for a rounded result.
fn main() {
    let sum = 0.1_f64 + 0.2;
    let expected = 0.3_f64;

    // This is why `assert_eq!(sum, expected)` would fail.
    assert_ne!(sum, expected);

    // Compare the values by their distance in representable values, or by an
    // absolute epsilon.
    assert!(float_cmp::approx_eq!(f64, sum, expected, ulps = 2));
    assert!(float_cmp::approx_eq!(
        f64,
        sum,
        expected,
        epsilon = f64::EPSILON
    ));

    // `approx` offers assertion macros, including relative comparisons.
    assert_relative_eq!(sum, expected, epsilon = f64::EPSILON);
}
// ANCHOR_END: example

/// Runs the example.
#[allow(clippy::main_recursion)]
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that ULP and epsilon comparisons accept the same rounded result.
    #[test]
    fn rounded_sum_is_approximately_equal() {
        main();
    }
}
