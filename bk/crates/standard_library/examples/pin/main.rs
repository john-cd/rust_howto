//! Runs the `Pin` example.
//!
//! TODO Reviewed ??-??-2026.

mod pin;

/// Runs the pin example.
fn main() {
    pin::run();
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
