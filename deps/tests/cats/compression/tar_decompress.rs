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

    Ok(())
}
// ANCHOR_END: example
