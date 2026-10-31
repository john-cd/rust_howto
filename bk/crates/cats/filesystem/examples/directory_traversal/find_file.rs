// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to traverse a directory and find files that
//! have been modified within the last 24 hours.
//!
//! Walking a directory tree visits every file below a path, and a filter
//! selects the ones you want by name or extension, like the `find` command.

use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use walkdir::WalkDir;

/// Walks the current directory tree and prints the files that were modified
/// recently.
fn main() -> Result<()> {
    for path in recently_modified(Path::new("."))? {
        println!("{}", path.display());
    }
    Ok(())
}

/// Returns the files under `root` modified within the last 24 hours.
fn recently_modified(root: &Path) -> Result<Vec<PathBuf>> {
    let mut found = Vec::new();
    // By default, WalkDir does not follow symbolic links to directories.
    for entry in WalkDir::new(root).sort_by_file_name() {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata().with_context(|| {
            format!("reading metadata for {}", path.display())
        })?;
        let modified = metadata.modified().with_context(|| {
            format!("reading modification time for {}", path.display())
        })?;

        // `elapsed` returns an error if the modification time is in the future.
        if modified
            .elapsed()
            .with_context(|| format!("computing age for {}", path.display()))?
            .as_secs()
            < 86_400
            && metadata.is_file()
        {
            found.push(path.to_path_buf());
        }
    }

    Ok(found)
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the example can traverse the current directory.
    /// Runs the find file example through its entry point.
    #[test]
    fn test_find_file_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// Only fresh files are returned; old files and directories are not.
    #[test]
    fn returns_only_recent_files() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let fresh = dir.path().join("fresh.txt");
        let old = dir.path().join("old.txt");
        std::fs::write(&fresh, "")?;
        std::fs::write(&old, "")?;
        let two_days_ago =
            std::time::SystemTime::now() - std::time::Duration::from_hours(48);
        std::fs::File::options()
            .write(true)
            .open(&old)?
            .set_modified(two_days_ago)?;
        std::fs::create_dir(dir.path().join("subdir"))?;

        assert_eq!(recently_modified(dir.path())?, vec![fresh]);
        Ok(())
    }

    /// A missing root is an error.
    #[test]
    fn missing_root_is_an_error() {
        assert!(recently_modified(Path::new("/definitely/not/here")).is_err());
    }
}
