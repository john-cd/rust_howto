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
    main()?;
    Ok(())
}
// TODO create a PNG file?
