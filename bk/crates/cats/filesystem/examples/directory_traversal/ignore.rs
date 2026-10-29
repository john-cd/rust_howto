// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to use the `ignore` crate to traverse a
//! directory recursively, respecting `.gitignore` and similar files, and
//! ignoring hidden files.
//!
//! In `Cargo.toml`, add:
//! ```toml
//! ignore = "0.4" # or latest.
//! ```

use ignore::WalkBuilder;

/// Walks directories while respecting the ignore files, such as `.gitignore`.
fn main() {
    // Specify the directory to walk.
    let root = std::path::Path::new("examples/");
    for line in walk(root, "temp/") {
        println!("{line}");
    }
}

/// Describes the files and directories under `root` and `extra`, skipping
/// whatever the ignore files exclude.
fn walk(root: &std::path::Path, extra: &str) -> Vec<String> {
    let mut lines = Vec::new();
    // Create a `WalkBuilder` and build a new `Walk` iterator.
    // A lot more options are available.
    let walker = WalkBuilder::new(root)
        .add(extra) // Add another directory to walk.
        .standard_filters(true) // Respect `.gitignore` and other ignore files.
        .build();

    // Iterate over the entries.
    for result in walker {
        match result {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() {
                    lines.push(format!("File: {path:?}"));
                } else if path.is_dir() {
                    lines.push(format!("Directory: {path:?}"));
                }
            }
            Err(err) => {
                eprintln!("Error: {err}");
            }
        }
    }
    lines
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The example's own tree can be walked.
    #[test]
    fn test_ignore_example_entry_point() {
        main();
    }

    /// Ignored files are skipped, others reported, and both roots are walked.
    #[test]
    fn respects_ignore_files() -> std::io::Result<()> {
        use std::fs;
        let dir = tempfile::tempdir()?;
        let (a, b) = (dir.path().join("a"), dir.path().join("b"));
        fs::create_dir(&a)?;
        fs::create_dir(&b)?;
        fs::write(a.join(".ignore"), "skipped.txt\n")?;
        fs::write(a.join("kept.txt"), "")?;
        fs::write(a.join("skipped.txt"), "")?;
        fs::write(b.join("other.txt"), "")?;

        let lines = walk(&a, b.to_str().expect("utf-8 path"));
        let joined = lines.join("\n");
        assert!(joined.contains("kept.txt"));
        assert!(joined.contains("other.txt"));
        assert!(!joined.contains("skipped.txt"));
        Ok(())
    }
}
