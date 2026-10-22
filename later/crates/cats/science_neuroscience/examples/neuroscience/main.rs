//! Runs the neuroscience example.
//!
//! TODO Reviewed ??-??-2026

mod neuro;

/// Runs the example from the `neuro` module.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    neuro::run()
}
