// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to traverse a directory and find files that
//! have been modified within the last 24 hours.

use anyhow::Context;
use anyhow::Result;
use walkdir::WalkDir;

/// Walks the current directory tree and prints the files that were modified
/// recently.
fn main() -> Result<()> {
    // By default, WalkDir does not follow symbolic links to directories.
    for entry in WalkDir::new(".") {
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
            println!("{}", path.display());
        }
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
    /// Runs the find file example through its entry point.
    #[test]
    fn test_find_file_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
