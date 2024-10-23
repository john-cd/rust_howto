use std::fs::File;

use flate2::read::GzDecoder;
use tar::Archive;

#[test]
#[ignore]
fn test() -> Result<(), std::io::Error> {
    let path = "temp/archive.tar.gz";

    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;

    Ok(())
}

// TODO remove dependency on tar.gz file
