//! Runs the `Default` trait example.
//!
//! TODO Reviewed ??-??-2026.

/// The `Default` trait provides a way to create a default value for a type.
mod default;
/// Runs the example.
/// Runs the `default` example.
fn main() {
    default::run();
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
