#![allow(dead_code)]
// ANCHOR: example
use std::fs::File;

use flate2::read::GzDecoder;
use tar::Archive;

pub fn main() -> Result<(), std::io::Error> {
    // Define the path to the compressed tar archive.
    // This file should exist in the 'temp' directory.
    // The archive is expected to be a gzip-compressed tar file.
    let path = "temp/archive.tar.gz";

    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    // The archive will be unpacked into the 'temp' directory.
    archive.unpack("temp")?;
    println!("archive file unpacked!");
    Ok(())
}
// ANCHOR_END: example

// [review; tar_decompress.rs is noplayground - fix?](https://github.com/john-cd/rust_howto/issues/254)
