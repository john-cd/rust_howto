//! Runs the satellite tracking example.
//!
//! TODO Reviewed ??-??-2026.

mod aero;

/// Runs the aerospace example.
fn main() {
    aero::run();
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
