//! Runs the emulator example.
//!
//! TODO Reviewed ??-??-2026.

mod intel_hex;
mod polkavm_host;
mod register_vm;

/// Runs the emulators example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    register_vm::run()?;
    polkavm_host::run()?;
    intel_hex::run()
}
