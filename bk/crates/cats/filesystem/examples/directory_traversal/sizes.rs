#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Calculates the total size of all files within a directory tree
//! (within a given depth).

use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;
use walkdir::WalkDir;

/// Calculates the total size of all files within a directory tree
/// (within a given depth).
fn main() -> Result<()> {
    let mut total_size = 0_u64;
    for entry in WalkDir::new(".").min_depth(1).max_depth(3) {
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

    println!("Total size: {total_size} bytes.");
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
    /// Runs the sizes example through its entry point.
    #[test]
    fn test_sizes_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
