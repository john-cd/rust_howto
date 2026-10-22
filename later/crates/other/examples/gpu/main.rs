//! Runs the GPU example.
//!
//! TODO Reviewed ??-??-2026

use anyhow::Result;

mod rust_gpu;

/// Runs the example from the `rust_gpu` module.
fn main() -> Result<()> {
    rust_gpu::run();
    Ok(())
}
