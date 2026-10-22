//! Runs the encoding example.
//!
//! TODO Reviewed ??-??-2026

mod encoding;

/// Runs the example from the `encoding` module.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    encoding::run()
}
