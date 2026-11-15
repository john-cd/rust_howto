//! Runs the units of measurement example.
//!
//! TODO Reviewed ??-??-2026.

/// Compute with quantities that carry units.
mod quantities_and_conversions;
/// Runs the example.
/// Runs the quantity conversion example.

fn main() {
    quantities_and_conversions::run();
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
