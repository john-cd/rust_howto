//! Runs the units of measurement example.
//!
//! TODO Reviewed ??-??-2026.

mod quantities_and_conversions;

/// Runs the quantity conversion example.
fn main() {
    quantities_and_conversions::run();
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
