#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example shows how to use the `exp` function on a complex number.

use std::f64::consts::PI;

use num::complex::Complex;

/// This example shows how to use the `exp` function on a complex number.
fn main() {
    let x = Complex::new(0.0, 2.0 * PI);

    println!("e^(2i * pi) = {}", x.exp()); // =~1
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the exponential example runs without panicking.
    #[test]
    fn test_main() {
        main();
    }
}
