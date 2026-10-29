//! Runs the no std example.
//!
//! TODO Reviewed ??-??-2026.

mod checksum;

/// Runs the no std example.
fn main() {
    checksum::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
