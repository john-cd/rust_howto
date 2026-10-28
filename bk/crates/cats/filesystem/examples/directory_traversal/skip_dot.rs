// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to skip hidden files and directories during
//! directory traversal.

use anyhow::Result;
use walkdir::DirEntry;
use walkdir::WalkDir;

/// Checks if a directory entry should be included in the traversal.
///
/// This function filters out hidden files and directories (those starting with
/// a dot) except for the root directory itself.
fn is_not_hidden(entry: &DirEntry) -> bool {
    entry.depth() == 0 || !entry.file_name().to_string_lossy().starts_with('.')
}

/// Prints the paths of the current directory tree, skipping hidden files and
/// directories.
fn main() -> Result<()> {
    let w = WalkDir::new(".");
    for entry in w.into_iter().filter_entry(is_not_hidden) {
        let entry = entry?;
        println!("{}", entry.path().display());
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

    /// Verifies the example can traverse the current directory.
    /// Runs the skip dot example through its entry point.
    #[test]
    fn test_skip_dot_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
