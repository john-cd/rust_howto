#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use the `glob` crate
//! to find all PNG files in a directory tree.

use anyhow::Result;
use glob::glob;

fn main() -> Result<()> {
    // Find all PNG files in the current directory and its subdirectories.
    for entry in glob("**/*.png")? {
        println!("{}", entry?.display());
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    // Create a temporary PNG file for testing.
    let path = std::path::Path::new("test_image.png");
    std::fs::File::create(path)?;

    main()?;

    // Clean up.
    std::fs::remove_file(path)?;

    Ok(())
}
