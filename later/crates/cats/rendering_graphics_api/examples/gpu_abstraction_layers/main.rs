//! Runs the gpu abstraction layers example.
//!
//! TODO Reviewed ??-??-2026.

use std::error::Error;

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

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    #[cfg_attr(feature = "wgpu", ignore = "needs a GPU adapter")]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
