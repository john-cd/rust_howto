//! Runs the discrete mathematics example.
//!
//! TODO Reviewed ??-??-2026.

/// Combine sets with union, intersection and difference.
mod set_operations;
/// Runs the example.
/// Runs the set operations example.
fn main() {
    set_operations::run();
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
