//! Runs the `accesskit` accessibility example.
//!
//! Reviewed 10-27-2026.

mod accesskit;

/// Runs the screen readers example.
fn main() {
    accesskit::run();
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
