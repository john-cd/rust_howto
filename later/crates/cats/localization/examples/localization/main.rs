//! Runs the localization example.
//!
//! TODO Reviewed ??-??-2026.

mod localization1;

/// Runs the localization example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    localization1::run()
}
