//! Runs the GPU example.
//!
//! TODO Reviewed ??-??-2026.

use anyhow::Result;

mod rust_gpu;

/// Runs the gpu example.
fn main() -> Result<()> {
    rust_gpu::run();
    Ok(())
}
