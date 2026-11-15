//! Runs the builder pattern example.
//!
//! TODO Reviewed ??-??-2026.

/// Example builder pattern.
mod builder_pattern;
/// Runs the example.
/// Runs the builder pattern example.
fn main() {
    builder_pattern::run();
}

/// Verifies that the builder pattern example.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
