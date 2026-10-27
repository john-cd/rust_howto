//! Runs the internationalization example.
//!
//! TODO Reviewed ??-??-2026.

mod icu_formatting;

/// Runs the internationalization example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    icu_formatting::run()
}
