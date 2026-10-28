// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Creates complex numbers with integer and floating-point components.

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
}
