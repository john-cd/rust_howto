#![allow(clippy::incompatible_msrv)]
// ANCHOR: example

use std::fs::create_dir;
use std::fs::exists;
use std::fs::File;

use flate2::write::GzEncoder;
use flate2::Compression;

pub fn main() -> Result<(), std::io::Error> {
    // Create a temporary folder
    if !exists("temp")? {
        create_dir("temp")?;
    }
    // Create the archive file
    let tar_gz_file = File::create("temp/archive.tar.gz")?;
    // Create the compression encoder
    let enc = GzEncoder::new(tar_gz_file, Compression::default());
    // Create a new archive builder with the underlying file as the
    // destination of all data written.
    let mut tar = tar::Builder::new(enc);
    // Archive the /var/log directory and all of its contents
    // (recursively), renaming it in the process
    tar.append_dir_all("temp/backup/var/log", "/var/log")?;
    Ok(())
}

// ANCHOR_END: example
