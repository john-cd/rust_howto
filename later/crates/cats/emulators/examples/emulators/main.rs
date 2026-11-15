//! Runs the emulator example.
//!
//! TODO Reviewed ??-??-2026.

/// Parses an Intel HEX data record and its end-of-file record.
mod intel_hex;
/// A simple `PolkaVM` host example.
mod polkavm_host;
/// A two-register virtual machine with a bounded fetch/decode/execute loop.
mod register_vm;

/// Runs the emulators example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    register_vm::run()?;
    polkavm_host::run()?;
    intel_hex::run()
}

/// test their own results).
#[cfg(test)]
mod tests {
    use super::*;

    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
