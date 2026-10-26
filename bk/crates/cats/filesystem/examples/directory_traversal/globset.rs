#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to use the `globset` crate to match file paths
//! against a set of glob patterns.

use std::fs;

use globset::Glob;
use globset::GlobSet;
use globset::GlobSetBuilder;

/// Prints the files of a directory that match a set of glob patterns.
fn main() -> anyhow::Result<()> {
    // Create a self-contained fixture so the example works from any directory.
    let temp_dir = tempfile::tempdir()?;
    fs::write(temp_dir.path().join("main.rs"), "fn main() {}\n")?;
    fs::write(temp_dir.path().join("README.md"), "Example fixture\n")?;
    fs::write(temp_dir.path().join("notes.txt"), "Not a match\n")?;

    // Create a glob set builder and add patterns.
    let mut builder = GlobSetBuilder::new();
    builder.add(Glob::new("*.rs")?);
    builder.add(Glob::new("*.md")?);
    let glob_set: GlobSet = builder.build()?;

    // Specify the directory to search.
    let dir = temp_dir.path();

    // Iterate over the files in the directory.
    let mut matched_count = 0;
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        // Check if the file matches any of the patterns.
        if glob_set.is_match(&path) {
            println!("Matched: {}", path.display());
            matched_count += 1;
        }
    }
    assert_eq!(matched_count, 2);

    // Compile a single glob when you do not need a set of patterns.
    let glob = Glob::new("*.rs")?.compile_matcher();
    assert!(glob.is_match("foo/bar.rs"));

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies matching results against a temporary fixture from any working
    /// directory.
    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }
}
