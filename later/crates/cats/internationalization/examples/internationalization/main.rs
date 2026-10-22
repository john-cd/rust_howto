//! Runs the internationalization example.
//!
//! TODO Reviewed ??-??-2026

mod internationalization1;

/// Runs the example from the `internationalization1` module.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    internationalization1::run()
}
