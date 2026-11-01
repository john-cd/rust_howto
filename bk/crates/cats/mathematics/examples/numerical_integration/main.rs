//! Runs the numerical integration example.
//!
//! TODO Reviewed ??-??-2026.

mod quadrature;

/// Prints the trapezoidal and Simpson estimates.
fn main() {
    quadrature::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example; each module's `run()` asserts its own results, so a
    /// panic fails the test.
    #[test]
    fn test_main_runs() {
        main();
    }
}
