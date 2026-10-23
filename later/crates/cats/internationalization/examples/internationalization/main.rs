//! Runs the internationalization example.
//!
//! TODO Reviewed ??-??-2026.

mod internationalization1;

/// Runs the internationalization example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    internationalization1::run()
}
