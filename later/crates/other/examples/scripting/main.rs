//! Runs the scripting example.
//!
//! TODO Reviewed ??-??-2026.

use anyhow::Result;
use anyhow::anyhow;

mod rhai;
mod rhai_types_and_limits;

/// Runs the scripting example.
fn main() -> Result<()> {
    rhai::run().map_err(|error| anyhow!(error.to_string()))?;
    rhai_types_and_limits::run().map_err(|error| anyhow!(error.to_string()))?;
    Ok(())
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
