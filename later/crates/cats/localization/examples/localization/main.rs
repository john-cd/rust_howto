//! Runs the localization example.
//!
//! TODO Reviewed ??-??-2026.

mod fluent;

/// Runs the localization example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    fluent::run()
}
