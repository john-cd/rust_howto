//! Runs the builder pattern example.
//!
//! TODO Reviewed ??-??-2026.

mod builder_pattern;

/// Runs the builder pattern example.
fn main() {
    builder_pattern::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    fn main_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
