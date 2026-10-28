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

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
