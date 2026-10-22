//! Runs the localization example.
//!
//! TODO Reviewed ??-??-2026

mod localization1;

/// Runs the example from the `localization1` module.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    localization1::run()
}
