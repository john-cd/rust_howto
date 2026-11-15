//! Runs the numerical integration example.
//!
//! TODO Reviewed ??-??-2026.

/// Demonstrates dependency-free numerical integration.
mod quadrature;

/// Prints the trapezoidal and Simpson estimates.
fn main() {
    quadrature::run();
}

/// panic fails the test.
#[cfg(test)]
mod tests {
    use super::*;

    /// panic fails the test.
    #[test]
    fn test_main_runs() {
        main();
    }
}
