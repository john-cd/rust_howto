// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates the manipulation of complex numbers.

/// Adds `(10 + 20i)` and `(3.1 - 4.2i)`.
fn sum() -> num::complex::Complex<f64> {
    // Create two complex numbers.
    let complex_num1 = num::complex::Complex::new(10.0, 20.0);
    let complex_num2 = num::complex::Complex::new(3.1, -4.2);

    // Add the two complex numbers.
    complex_num1 + complex_num2
}

/// Adds two complex numbers and prints the sum.
fn main() {
    let sum = sum();

    println!("Sum: {sum}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the addition example runs without panicking.
    /// Runs the add complex example through its entry point.
    #[test]
    fn test_add_complex_example_entry_point() {
        main();
    }

    /// Real and imaginary parts add independently; addition is commutative.
    #[test]
    fn adds_componentwise() {
        let s = sum();
        assert!((s.re - 13.1).abs() < 1e-12);
        assert!((s.im - 15.8).abs() < 1e-12);
        let a = num::complex::Complex::new(1.0, 2.0);
        let b = num::complex::Complex::new(-3.0, 0.5);
        assert_eq!(a + b, b + a);
        assert_eq!(a + num::complex::Complex::new(0.0, 0.0), a);
    }
}
