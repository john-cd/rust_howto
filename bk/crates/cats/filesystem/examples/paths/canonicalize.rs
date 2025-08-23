#![allow(dead_code)]
// ANCHOR: example
use std::fs;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let path = "./temp/examples";
    fs::create_dir_all(path)?;

    // Return the canonical, absolute form of a path with all intermediate
    // components normalized and symbolic links resolved. The path must
    // exist.
    let normalized_path: PathBuf =
        fs::canonicalize("./temp/examples/../examples")?;
    println!("Canonical path: {}", normalized_path.display());

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
