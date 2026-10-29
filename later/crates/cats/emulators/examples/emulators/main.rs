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

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
