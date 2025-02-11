// ANCHOR: example
use std::f64;

use approx::assert_abs_diff_eq;
use approx::assert_relative_eq;
use approx::assert_ulps_eq;
use approx::relative_eq;

// `approx` provides facilities for testing the approximate equality of
// floating-point based types, which are subject to rounding errors.
// It is commonly used in tests.
fn main() {
    let a = 1.0000001;
    let b = 1.0000002;

    // Use `assert_relative_eq` to compare the two values approximately.
    // It panics with a helpful error on failure.
    // You can specifiy the tolerance to use, and other parameters.
    assert_relative_eq!(a, b, epsilon = 1.0e-6);
    println!("The numbers are approximately equal!");

    // The assertion delegates to an approximative equality macro.
    let _result: bool = relative_eq!(a, b);

    // Other methods of comparing floating-point numbers are available:

    // Approximate equality of using the absolute difference:
    assert_abs_diff_eq!(1.0, 1.0);
    assert_abs_diff_eq!(1.0, 1.0, epsilon = f64::EPSILON);

    // Approximate equality using both the absolute difference and ULPs (Units
    // in Last Place):
    assert_ulps_eq!(1.0, 1.0);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
