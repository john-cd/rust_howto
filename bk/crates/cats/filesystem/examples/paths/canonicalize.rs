// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Canonicalizing paths.
//!
//! Canonicalizing turns a path into an absolute one with `.`, `..` and symbolic
//! links resolved. The file must exist, because the file system is consulted.
//! On Windows the result has a `\\?\` prefix.

use std::fs;
use std::path::PathBuf;

/// Canonicalizes a path with `..` components.
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

pub fn run() -> std::io::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the canonicalize example through its entry point.
    #[test]
    fn test_canonicalize_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
