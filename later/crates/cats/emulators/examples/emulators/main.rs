//! Runs the emulator example.
//!
//! TODO Reviewed ??-??-2026.

mod emulator;
mod intel_hex;

/// Runs the emulators example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    emulator::run()?;
    intel_hex::run()
}
