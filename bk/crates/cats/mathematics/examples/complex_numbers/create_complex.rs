// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Creates complex numbers with integer and floating-point components.
//!
//! A complex number has a real part and an imaginary part, written a + bi,
//! where i is the square root of -1. They appear in signal processing,
//! electronics and graphics.

/// Creates complex numbers with integer and floating-point components.
fn main() {
    // Create a complex number with integer components.
    let complex_integer = num::complex::Complex::new(10, 20);

    // Create a complex number with floating-point components.
    let complex_float = num::complex::Complex::new(10.1, 20.1);

    println!("Complex integer: {complex_integer}");
    println!("Complex float: {complex_float}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the construction example runs without panicking.
    /// Runs the create complex example through its entry point.
    #[test]
    fn test_create_complex_example_entry_point() {
        main();
    }

    /// Components are stored as given and displayed as `re+imi`.
    #[test]
    fn stores_and_displays_components() {
        let c = num::complex::Complex::new(10, 20);
        assert_eq!((c.re, c.im), (10, 20));
        assert_eq!(c.to_string(), "10+20i");
        assert_eq!(
            num::complex::Complex::new(10.1, -20.1).to_string(),
            "10.1-20.1i"
        );
        assert_eq!(
            num::complex::Complex::new(0, 0),
            num::complex::Complex::default()
        );
    }
}
