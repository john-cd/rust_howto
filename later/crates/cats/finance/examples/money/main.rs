//! Runs the money examples.
//!
//! TODO Reviewed ??-??-2026.

mod rusty_money;

/// Runs the money examples.
fn main() {
    rusty_money::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the module
    /// tests its own results).
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
