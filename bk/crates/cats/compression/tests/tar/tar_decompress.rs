// ANCHOR: example
use std::fs::File;

use flate2::read::GzDecoder;
use tar::Archive;

pub fn main() -> Result<(), std::io::Error> {
    let path = "temp/archive.tar.gz";

    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;
    println!("archive file unpacked!");
    Ok(())
}
// ANCHOR_END: example

// [WIP review; tar_decompress.rs is noplayground - fix?](https://github.com/john-cd/rust_howto/issues/254)
