// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to skip hidden files and directories during
//! directory traversal.

use std::path::Path;
use std::path::PathBuf;

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
    for path in visible_paths(Path::new("."))? {
        println!("{}", path.display());
    }
    Ok(())
}

/// Returns the paths under `root`, skipping hidden entries.
fn visible_paths(root: &Path) -> Result<Vec<PathBuf>> {
    let w = WalkDir::new(root).sort_by_file_name();
    let mut paths = Vec::new();
    for entry in w.into_iter().filter_entry(is_not_hidden) {
        paths.push(entry?.into_path());
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

    /// Verifies the example can traverse the current directory.
    /// Runs the skip dot example through its entry point.
    #[test]
    fn test_skip_dot_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// Hidden files and the contents of hidden directories are skipped.
    #[test]
    fn skips_hidden_entries() -> Result<()> {
        let dir = tempfile::tempdir()?;
        std::fs::create_dir(dir.path().join(".git"))?;
        std::fs::write(dir.path().join(".git/config"), "")?;
        std::fs::write(dir.path().join(".hidden"), "")?;
        std::fs::write(dir.path().join("shown"), "")?;
        assert_eq!(
            visible_paths(dir.path())?,
            vec![dir.path().to_path_buf(), dir.path().join("shown")]
        );
        Ok(())
    }

    /// A hidden root directory is still walked (depth 0 is exempt).
    #[test]
    fn hidden_root_is_still_walked() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let root = dir.path().join(".root");
        std::fs::create_dir(&root)?;
        std::fs::write(root.join("f"), "")?;
        assert_eq!(visible_paths(&root)?.len(), 2);
        Ok(())
    }
}
