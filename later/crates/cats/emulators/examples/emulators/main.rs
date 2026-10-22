//! Runs the emulator example.
//!
//! TODO Reviewed ??-??-2026

mod emulator;

/// Runs the example from the `emulator` module.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    emulator::run()
}
