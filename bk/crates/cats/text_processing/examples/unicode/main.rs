//! Runs the Unicode grapheme example.
//!
//! TODO Reviewed ??-??-2026.

mod graphemes;

/// Runs the unicode example.
fn main() {
    graphemes::run();
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
