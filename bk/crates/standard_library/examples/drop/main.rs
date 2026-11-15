//! Runs the `Drop` trait example.
//!
//! TODO Reviewed ??-??-2026.

/// Running code when a value is dropped, with the `Drop` trait.
mod drop;
/// Runs the example.
/// Runs the drop example.

fn main() {
    drop::run();
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
