//! Runs the multimedia example.
//!
//! TODO Reviewed ??-??-2026

mod multimedia1;

/// Runs the example from the `multimedia1` module.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    multimedia1::run()
}
