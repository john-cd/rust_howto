// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Find duplicate filenames in the current directory.

use std::collections::HashMap;
use std::ffi::OsString;
use std::path::Path;

use anyhow::Result;
use walkdir::WalkDir;

/// Prints the file names that appear more than once in the current directory
/// tree.
fn main() -> Result<()> {
    for name in duplicate_names(Path::new("."))? {
        println!("{}", name.to_string_lossy());
    }
    Ok(())
}

/// Returns the file names that appear more than once under `root`, in the
/// order the second occurrence is found.
fn duplicate_names(root: &Path) -> Result<Vec<OsString>> {
    let mut filenames = HashMap::<OsString, usize>::new();
    let mut duplicates = Vec::new();

    for entry in WalkDir::new(root).sort_by_file_name() {
        let entry = entry?;
        if entry.file_type().is_dir() {
            continue;
        }

        let f_name = entry.file_name().to_os_string();
        let counter = filenames.entry(f_name.clone()).or_insert(0);
        *counter += 1;

        if *counter == 2 {
            duplicates.push(f_name);
        }
    }
    Ok(duplicates)
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the example can traverse the current directory.
    /// Runs the duplicate name example through its entry point.
    #[test]
    fn test_duplicate_name_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// Names repeated in different directories are reported exactly once.
    #[test]
    fn reports_repeated_names_once() -> Result<()> {
        let dir = tempfile::tempdir()?;
        for sub in ["a", "b", "c"] {
            std::fs::create_dir(dir.path().join(sub))?;
            std::fs::write(dir.path().join(sub).join("same.txt"), "")?;
        }
        std::fs::write(dir.path().join("a/unique.txt"), "")?;
        assert_eq!(
            duplicate_names(dir.path())?,
            vec![OsString::from("same.txt")]
        );
        Ok(())
    }

    /// An empty tree has no duplicates, and a missing root is an error.
    #[test]
    fn empty_tree_and_missing_root() -> Result<()> {
        let dir = tempfile::tempdir()?;
        assert!(duplicate_names(dir.path())?.is_empty());
        assert!(duplicate_names(&dir.path().join("missing")).is_err());
        Ok(())
    }
}
