#![allow(dead_code)]
#![allow(clippy::incompatible_msrv)]
// ANCHOR: example
use std::fs;
use std::fs::File;

use flate2::Compression;
use flate2::write::GzEncoder;

/// This function creates a compressed tar archive (tar.gz) of the 'tests'
/// directory.
pub fn main() -> Result<(), std::io::Error> {
    // Create a temporary folder:
    if !fs::exists("temp")? {
        fs::create_dir("temp")?;
    }
    // Create the archive file:
    let tar_gz_file = File::create("temp/archive.tar.gz")?;
    // Create the compression encoder:
    let enc = GzEncoder::new(tar_gz_file, Compression::default());
    // Create a new archive builder with the underlying file as the
    // destination of all data written.
    let mut tar = tar::Builder::new(enc);
    // Archive a directory and all of its contents
    // (recursively), renaming it in the process:
    tar.append_dir_all("backup", "./temp")?;
    println!("`archive.tar.gz` file created!");
    Ok(())
}
// ANCHOR_END: example

// [review; tar_compress.rs is noplayground - fix?](https://github.com/john-cd/rust_howto/issues/255)
