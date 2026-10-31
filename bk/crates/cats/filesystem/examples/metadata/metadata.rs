// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Reading the metadata of files and directories.
//!
//! File metadata is information about a file apart from its content: size,
//! type, permissions and times (modified, accessed and, on some systems,
//! created). It comes from the file system.

use std::fs;
use std::time::UNIX_EPOCH;

/// Prints the metadata of the file or directory at `path`.
fn inspect_file<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<()> {
    let path = path.as_ref();

    // Attempt to get metadata for the given path:
    let metadata = fs::metadata(path)?;

    println!("Metadata for: {}", path.display());
    println!(
        "  Type: {}",
        if metadata.is_dir() {
            "Directory"
        } else if metadata.is_file() {
            "File"
        } else {
            "Other"
        }
    );
    println!("  Size: {} bytes", metadata.len());
    println!("  Read-only: {}", metadata.permissions().readonly());

    // Handle the modification time:
    match metadata.modified() {
        Ok(mod_time) => {
            let duration = mod_time.duration_since(UNIX_EPOCH)?;
            println!(
                "Last modified: {} seconds since Unix epoch",
                duration.as_secs()
            );
        }
        Err(e) => eprintln!("Could not retrieve modification time: {e}"),
    }

    Ok(())
}

/// Creates files and directories and prints their metadata.
fn main() -> anyhow::Result<()> {
    inspect_tree(std::path::Path::new("."))
}

/// Creates files and directories under `base`, prints their metadata, and
/// cleans up.
fn inspect_tree(base: &std::path::Path) -> anyhow::Result<()> {
    // Create a few folders and files for the next step.
    let path = base.join("temp/files/subfolder");
    fs::create_dir_all(path)?; // Creates 'temp', 'temp/files', and 'temp/files/subfolder' if they don't exist.
    fs::write(base.join("temp/files/file1.txt"), "Hello")?;
    fs::write(base.join("temp/files/subfolder/file2.log"), "World")?;

    // Show permissions for files and folders directly within `temp/files/`.
    // `read_dir` returns an iterator over the directory's entries.
    // `flatten` is used to skip over any entries that resulted in an error.
    for entry in (fs::read_dir(base.join("temp/files"))?).flatten() {
        let path = entry.path();
        inspect_file(path)?;
        // You could also use `entry.metadata()` directly in this case.
    }
    fs::remove_dir_all(base.join("temp/files"))?;
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the metadata example through its entry point.
    #[test]
    fn test_metadata_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// The tree is created, inspected and removed under the base directory.
    #[test]
    fn inspects_and_cleans_up_tree() -> anyhow::Result<()> {
        let base = tempfile::tempdir()?;
        inspect_tree(base.path())?;
        assert!(base.path().join("temp").is_dir());
        assert!(!base.path().join("temp/files").exists());
        Ok(())
    }

    /// Files report their size; missing paths are errors.
    #[test]
    fn inspect_file_handles_files_and_missing_paths() -> anyhow::Result<()> {
        let base = tempfile::tempdir()?;
        let file = base.path().join("f.txt");
        fs::write(&file, "12345")?;
        inspect_file(&file)?;
        assert_eq!(fs::metadata(&file)?.len(), 5);
        inspect_file(base.path())?;
        assert!(inspect_file(base.path().join("missing")).is_err());
        Ok(())
    }
}
