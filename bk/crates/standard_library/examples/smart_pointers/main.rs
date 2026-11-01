//! Runs the smart pointer example.
//!
//! TODO Reviewed ??-??-2026.

mod deref;

/// Runs the smart pointers example.
fn main() {
    deref::run();
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
