#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Example: create a dummy neuroimaging volume and save it as a `NIfTI` file.
//!
//! This example uses the `nifti` crate to build a synthetic 3D brain-like
//! volume, then writes it to the system temporary directory.

use std::error::Error;
use std::path::PathBuf;

use ndarray::Array3;
use nifti::writer::WriterOptions;

/// Builds a process-specific output path in the system temporary directory.
fn output_path() -> PathBuf {
    std::env::temp_dir().join(format!(
        "rust_howto_neuro_example_{}.nii",
        std::process::id()
    ))
}

/// Creates a synthetic volume of neuroimaging data and prints its location and
/// shape.
///
/// # Errors
///
/// Returns an error if the synthetic NIfTI volume cannot be written.
fn main() -> Result<(), Box<dyn Error>> {
    let shape = (64, 64, 64);
    let mut volume = Array3::<i16>::zeros(shape);

    for ((x, y, z), pixel) in volume.indexed_iter_mut() {
        let dx = x as f32 - 32.0;
        let dy = y as f32 - 32.0;
        let dz = z as f32 - 32.0;
        let radius = (dx * dx + dy * dy + dz * dz).sqrt();
        *pixel = ((radius.sin() * 40.0 + 80.0).clamp(0.0, 255.0)) as i16;
    }

    let out_path = output_path();
    WriterOptions::new(&out_path).write_nifti(&volume)?;
    println!(
        "Saved a synthetic NIfTI volume to {} with shape {:?}",
        out_path.display(),
        shape
    );
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<(), Box<dyn Error>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the example writes a NIfTI file to the temporary directory.
    #[test]
    fn test_main() -> Result<(), Box<dyn Error>> {
        let path = output_path();
        main()?;
        assert!(path.is_file(), "expected output file at {}", path.display());
        std::fs::remove_file(path)?;
        Ok(())
    }
    // [review](https://github.com/john-cd/rust_howto/issues/840)
}
