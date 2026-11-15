//! Runs the scripting example.
//!
//! TODO Reviewed ??-??-2026.

use anyhow::Result;
use anyhow::anyhow;

/// # Rhai scripting Rhai is a lightweight, Rust-based scripting language.
mod rhai;
/// Expose a Rust type to Rhai, keep state in a `Scope` and sandbox scripts.
mod rhai_types_and_limits;

/// Runs the scripting example.
fn main() -> Result<()> {
    rhai::run().map_err(|error| anyhow!(error.to_string()))?;
    rhai_types_and_limits::run().map_err(|error| anyhow!(error.to_string()))?;
    Ok(())
}
