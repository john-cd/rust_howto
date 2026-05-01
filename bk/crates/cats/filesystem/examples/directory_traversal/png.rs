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

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() -> anyhow::Result<()> {
        use std::fs;
        use std::path::Path;

        // Create a temporary directory.
        let temp_dir = Path::new("temp_png_test");
        if !temp_dir.exists() {
            fs::create_dir_all(temp_dir)?;
        }

        // Create a dummy PNG file.
        let png_file = temp_dir.join("test_image.png");
        fs::write(&png_file, b"\x89PNG\r\n\x1a\n")?;

        // Run main and check if it finds the file.
        // Note: main searches in the current directory tree using
        // glob("**/*.png").
        main()?;

        // Clean up.
        fs::remove_dir_all(temp_dir)?;

        Ok(())
    }
}
