//! Runs the `FromStr` parsing example.
//!
//! TODO Reviewed ??-??-2026.

mod from_str;

/// Runs the string parsing example.
fn main() {
    from_str::run();
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
