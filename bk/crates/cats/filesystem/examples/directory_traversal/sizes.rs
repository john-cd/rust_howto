// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Calculates the total size of all files within a directory tree
//! (within a given depth).

use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;
use walkdir::WalkDir;

/// Calculates the total size of all files within a directory tree
/// (within a given depth).
fn main() -> Result<()> {
    let total_size = total_size(Path::new("."))?;
    println!("Total size: {total_size} bytes.");
    Ok(())
}

/// Sums the sizes of the files at depth 1 to 3 under `root`.
fn total_size(root: &Path) -> Result<u64> {
    let mut total_size = 0_u64;
    for entry in WalkDir::new(root).min_depth(1).max_depth(3) {
        let entry = entry?;
        if entry.file_type().is_file() {
            total_size = total_size
                .checked_add(
                    entry
                        .metadata()
                        .with_context(|| {
                            format!(
                                "reading metadata for {}",
                                entry.path().display()
                            )
                        })?
                        .len(),
                )
                .ok_or_else(|| anyhow!("total file size exceeds u64::MAX"))?;
        }
    }

    Ok(total_size)
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the example can traverse the current directory.
    /// Runs the sizes example through its entry point.
    #[test]
    fn test_sizes_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// Sizes are summed to depth 3 only; directories do not count.
    #[test]
    fn sums_file_sizes_to_depth_three() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let deep = dir.path().join("a/b/c");
        std::fs::create_dir_all(&deep)?;
        std::fs::write(dir.path().join("top.txt"), "1234")?; // depth 1
        std::fs::write(dir.path().join("a/b/mid.txt"), "12")?; // depth 3
        std::fs::write(deep.join("deep.txt"), "123456789")?; // depth 4: excluded
        assert_eq!(total_size(dir.path())?, 6);
        assert_eq!(total_size(&dir.path().join("a/b/c"))?, 9);
        assert!(total_size(&dir.path().join("missing")).is_err());
        Ok(())
    }
}
