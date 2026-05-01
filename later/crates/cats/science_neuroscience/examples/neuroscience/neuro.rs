#![allow(dead_code)]
// ANCHOR: example
//! Example: create a dummy neuroimaging volume and save it as a NIfTI file.
//!
//! This example uses the `nifti` crate to build a synthetic 3D brain-like
//! volume, then writes it to the system temporary directory.

use std::error::Error;
use std::path::PathBuf;

use ndarray::Array3;
use nifti::NiftiHeader;
use nifti::writer::write_nifti;

fn main() -> Result<(), Box<dyn Error>> {
    let shape = (64, 64, 64);
    let mut volume = Array3::<i16>::zeros(shape);

    for ((x, y, z), pixel) in volume.indexed_iter_mut() {
        let dx = x as f32 - 32.0;
        let dy = y as f32 - 32.0;
        let dz = z as f32 - 32.0;
        let radius = (dx * dx + dy * dy + dz * dz).sqrt();
        *pixel = ((radius.sin() * 40.0 + 80.0).max(0.0).min(255.0)) as i16;
    }

    let header = NiftiHeader::default();

    let mut out_path = PathBuf::from(std::env::temp_dir());
    out_path.push("rust_howto_neuro_example.nii");
    write_nifti(&out_path, &volume, Some(&header))?;

    println!("Wrote synthetic NIfTI volume to {}", out_path.display());
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<(), Box<dyn Error>> {
    main()
}

#[test]
fn test() -> Result<(), Box<dyn Error>> {
    main()
}
// [review](https://github.com/john-cd/rust_howto/issues/840)
