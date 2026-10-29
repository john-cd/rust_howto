// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example shows how to use the `exp` function on a complex number.

use std::f64::consts::PI;

use num::complex::Complex;

/// Computes `e^(2i * pi)`, which is approximately 1 (Euler's identity).
fn euler() -> Complex<f64> {
    Complex::new(0.0, 2.0 * PI).exp()
}

/// This example shows how to use the `exp` function on a complex number.
fn main() {
    println!("e^(2i * pi) = {}", euler()); // =~1
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the exponential example runs without panicking.
    /// Runs the mathematical functions example through its entry point.
    #[test]
    fn test_mathematical_functions_example_entry_point() {
        main();
    }

    /// `e^(2 pi i)` is 1 up to rounding error; `e^0` is exactly 1.
    #[test]
    fn euler_identity_holds() {
        let e = euler();
        assert!((e.re - 1.0).abs() < 1e-12);
        assert!(e.im.abs() < 1e-12);
        assert_eq!(Complex::new(0.0, 0.0).exp(), Complex::new(1.0, 0.0));
        let half_turn = Complex::new(0.0, PI).exp();
        assert!((half_turn.re + 1.0).abs() < 1e-12);
    }
}
