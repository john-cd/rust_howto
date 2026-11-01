// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to use the `glob` crate
//! to find all PNG files in a directory tree.
//!
//! A glob such as `**/*.png` matches files by extension at any depth. Checking
//! the extension is quick, but does not prove the content is a PNG image.

use std::path::PathBuf;

use anyhow::Result;
use glob::glob;

/// Prints the paths of all the PNG files in the current directory tree.
fn main() -> Result<()> {
    // Find all PNG files in the current directory and its subdirectories.
    for path in find_pngs("**/*.png")? {
        println!("{}", path.display());
    }

    Ok(())
}

/// Returns the paths that match the glob `pattern`.
fn find_pngs(pattern: &str) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    for entry in glob(pattern)? {
        paths.push(entry?);
    }
    Ok(paths)
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The example runs against the current tree.
    #[test]
    fn test_png_example_entry_point() -> Result<()> {
        main()
    }

    /// Only PNG files at any depth match; an invalid pattern is an error.
    #[test]
    fn finds_png_files_recursively() -> Result<()> {
        let dir = tempfile::tempdir()?;
        std::fs::create_dir(dir.path().join("sub"))?;
        std::fs::write(dir.path().join("a.png"), "")?;
        std::fs::write(dir.path().join("sub/b.png"), "")?;
        std::fs::write(dir.path().join("c.txt"), "")?;
        let pattern = format!("{}/**/*.png", dir.path().display());
        let mut found = find_pngs(&pattern)?;
        found.sort();
        assert_eq!(
            found,
            vec![dir.path().join("a.png"), dir.path().join("sub/b.png")]
        );
        assert!(find_pngs("[").is_err());
        Ok(())
    }
}
