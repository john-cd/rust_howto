#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! The `num_traits` crate provides numeric traits for generic mathematics.

use num_traits::Num;
use num_traits::NumCast;
use num_traits::One;
use num_traits::Zero;

/// Adds a slice of values whose type implements the `Num` trait.
fn sum_values<T: Num + Copy>(values: &[T]) -> T {
    values
        .iter()
        .copied()
        .fold(T::zero(), |sum, value| sum + value)
}

/// Uses the `Zero`, `One` and `NumCast` traits on a number.
fn main() {
    let integer_sum = sum_values(&[1_i32, 2, 3]);
    let float_sum = sum_values(&[1.5_f64, 2.5]);
    assert_eq!(integer_sum, 6);
    assert_eq!(float_sum, 4.0);

    // Create a variable with an initial value of zero.
    let mut x: i32 = Zero::zero();
    println!("Initial value of x: {x}");
    assert_eq!(x, 0);

    // Set x to one using the `One` trait:
    x = One::one();
    println!("Value of x after setting to one: {x}");
    assert_eq!(x, 1);

    // Perform a type conversion using `NumCast`:
    let y: f64 =
        NumCast::from(x).expect("the num traits example invariant should hold");
    println!("Value of y after converting from x: {y}");
    assert_eq!(y, 1.0);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the generic numeric-trait example with integer and floating-point
    /// types.
    #[test]
    fn test_main() {
        main();
    }
}
