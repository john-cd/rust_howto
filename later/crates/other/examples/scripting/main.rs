//! Runs the scripting example.
//!
//! TODO Reviewed ??-??-2026

use anyhow::Result;
use anyhow::anyhow;

mod rhai;

fn main() -> Result<()> {
    rhai::run().map_err(|error| anyhow!(error.to_string()))?;
    Ok(())
}
