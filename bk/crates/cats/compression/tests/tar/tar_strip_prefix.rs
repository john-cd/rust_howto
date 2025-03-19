// ANCHOR: example
use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;
use flate2::read::GzDecoder;
use tar::Archive;

pub fn main() -> Result<()> {
    let file = File::open("temp/archive.tar.gz")?;
    let mut archive = Archive::new(GzDecoder::new(file));
    let prefix = "bundle/logs";

    println!("Extracted the following files:");
    archive
        .entries()?
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf> {
            let path = entry.path()?.strip_prefix(prefix)?.to_owned();
            entry.unpack(&path)?;
            Ok(path)
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));

    Ok(())
}
// ANCHOR_END: example

// [WIP review; tar_strip_prefix.rs is noplayground - fix?](https://github.com/john-cd/rust_howto/issues/256)
