// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to traverse a directory and check the last
//! modified time of each file.

use std::env;
use std::fs;
use std::path::Path;

use anyhow::Result;
use anyhow::anyhow;

/// Checks whether the files in the current directory were modified in the last
/// 24 hours.
fn main() -> Result<()> {
    // Get the current directory.
    let current_dir = env::current_dir()?;
    println!("Entries modified in the last 24 hours in {current_dir:?}:");
    for line in recent_entries(&current_dir)? {
        println!("{line}");
    }
    Ok(())
}

/// Describes the files in `dir` that were modified in the last 24 hours.
fn recent_entries(dir: &Path) -> Result<Vec<String>> {
    let mut lines = Vec::new();
    // Iterate over the entries within a directory.
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::metadata(&path)?;
        if let Ok(time) = metadata.modified() {
            // Returns the difference from the file's last modification time to
            // the current clock time. Note: SystemTime.elapsed can
            // be flaky.
            if let Ok(duration) = time.elapsed() {
                let last_modified = duration.as_secs();
                if (last_modified < 24 * 3600) && metadata.is_file() {
                    lines.push(format!(
                        "Last modified: {last_modified:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                        metadata.permissions().readonly(),
                        metadata.len(),
                        path.file_name().ok_or(anyhow!("No filename."))?
                    ));
                }
            }
        } else {
            lines.push(
                "Last modification time is not supported on this platform."
                    .to_owned(),
            );
        }
    }
    Ok(lines)
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the modified example through its entry point.
    #[test]
    fn test_modified_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// A fresh file is described with its size and name; subdirectories are
    /// skipped.
    #[test]
    fn describes_fresh_files() -> Result<()> {
        let dir = tempfile::tempdir()?;
        fs::write(dir.path().join("a.txt"), "12345")?;
        fs::create_dir(dir.path().join("sub"))?;
        let lines = recent_entries(dir.path())?;
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("size: 5 bytes"));
        assert!(lines[0].contains("a.txt"));
        Ok(())
    }

    /// An empty or missing directory behaves sensibly.
    #[test]
    fn empty_and_missing_directories() -> Result<()> {
        let dir = tempfile::tempdir()?;
        assert!(recent_entries(dir.path())?.is_empty());
        assert!(recent_entries(&dir.path().join("missing")).is_err());
        Ok(())
    }
}
