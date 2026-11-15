//! Runs the gpu abstraction layers example.
//!
//! TODO Reviewed ??-??-2026.

use std::error::Error;

/// Runs a headless `wgpu` compute shader that doubles each storage-buffer
#[cfg(feature = "wgpu")]
mod wgpu;

/// Runs the example from the `wgpu` module where it is supported.
fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "wgpu")]
    {
        wgpu::run()?;
    }

    Ok(())
}
