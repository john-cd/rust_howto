// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Add to `Cargo.toml`:
//! ```toml
//! remove_dir_all = { version = "1.0.0", features = [ "parallel" ] }
//! ```

use std::io;
use std::path::Path;

use remove_dir_all::remove_dir_all;

/// Creates a directory tree, then removes it with all its content.
fn main() -> io::Result<()> {
    remove_tree(Path::new("temp"))
}

/// Creates a directory tree under `base`, then removes a subdirectory.
fn remove_tree(base: &Path) -> io::Result<()> {
    // Create a directory structure for demonstration purposes
    // (if it doesn't exist already).
    let path = base.join("example_dir/sub_dir");
    std::fs::create_dir_all(&path)?;
    std::fs::write(base.join("example_dir/file1.txt"), "Hello, world!")?;
    std::fs::write(
        base.join("example_dir/sub_dir/file2.txt"),
        "Rust is awesome!",
    )?;

    // Now, remove the subdirectory and all of its contents.
    match remove_dir_all(&path) {
        Ok(()) => println!(
            "Directory 'sub_dir' and all contents removed successfully!"
        ),
        Err(e) => eprintln!("Failed to remove directory: {e}"),
    }
    // Beware:
    // - Calling this on a non-directory (e.g. a symlink to a directory)
    // will error.
    // - The caller must have permission to delete the files. The crate does
    // not change ACLs or file permissions to make deletion possible.

    // Security note: The functions `remove_dir_all`, and related
    // `remove_dir_contents`, and `ensure_empty_dir` are intrinsically
    // sensitive to file system races. The crate authors recommend using the
    // `RemoveDir` trait instead. This allows callers to be more confident
    // that what is deleted is what was requested even in the presence of
    // (malicious) actors changing the filesystem concurrently:
    //
    // let mut dir = std::fs::File::open("temp/")?;
    // use remove_dir_all::RemoveDir;
    // dir.remove_dir_contents(None)?;
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> io::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the remove dir all example through its entry point.
    #[test]
    fn test_remove_dir_all_example_entry_point() -> io::Result<()> {
        main()?;
        Ok(())
    }

    /// The subdirectory is removed; its sibling file remains.
    #[test]
    fn removes_subdirectory_but_keeps_siblings() -> io::Result<()> {
        let base = tempfile::tempdir()?;
        remove_tree(base.path())?;
        assert!(!base.path().join("example_dir/sub_dir").exists());
        assert!(base.path().join("example_dir/file1.txt").is_file());
        Ok(())
    }

    /// Removing a missing directory is an error from the crate.
    #[test]
    fn removing_missing_directory_fails() {
        let base = tempfile::tempdir().unwrap();
        assert!(remove_dir_all(base.path().join("missing")).is_err());
    }
}
