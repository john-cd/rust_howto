//! Runs the internationalization example.
//!
//! TODO Reviewed ??-??-2026.

mod icu_formatting;

/// Runs the internationalization example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    icu_formatting::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
