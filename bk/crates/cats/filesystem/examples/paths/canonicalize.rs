#![allow(dead_code)]
// ANCHOR: example
use std::fs;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    // Create a directory for this example.
    let path = "./temp/examples";
    fs::create_dir_all(path)?;

    // Canonicalize a path.
    // The path must exist.
    let normalized_path: PathBuf =
        fs::canonicalize("./temp/examples/../examples")?;
    println!("Canonical path: {}", normalized_path.display());

    // You may also use `Path::canonicalize`:
    // let path = std::path::Path::new("/tmp/xmpl/../xmpl/bar.rs");
    // assert_eq!(path.canonicalize().unwrap(),
    // PathBuf::from("/tmp/xmpl/bar.rs"));

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
