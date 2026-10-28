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

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
