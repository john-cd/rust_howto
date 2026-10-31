// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Creating, listing and removing directories.
//!
//! `create_dir_all` creates a directory and any missing parents, like `mkdir
//! -p`, and does not fail if the directory exists. `remove_dir_all` deletes a
//! directory tree for good, so double-check the path.

use std::fs;
use std::path::Path;

/// Creates, lists and removes directories under the current directory.
fn main() -> std::io::Result<()> {
    manipulate_dirs(Path::new("."))
}

/// Creates, lists and removes directories under `base`.
fn manipulate_dirs(base: &Path) -> std::io::Result<()> {
    // 1. Create a directory:
    let path = base.join("example_dir");
    if path.exists() {
        println!("Directory 'example_dir' already exists.");
    } else {
        fs::create_dir(&path)?;
        // This function will return an error e.g. if the parent directory
        // doesn't exist, if the directory already exists, or if
        // permissions are lacking.
        println!("Directory 'example_dir' created successfully!");
    }

    // 2. Remove it. The directory must be empty:
    fs::remove_dir(&path)?;
    println!("'{}' successfully removed.", path.display());

    // 3. Create a directory and any of its missing parents:
    let path = base.join("temp/my_files/subfolder");
    fs::create_dir_all(&path)?; // Creates 'temp', 'temp/my_files', and 'temp/my_files/subfolder' if they don't exist.
    // You can also use `DirBuilder`:
    // std::fs::DirBuilder::new()
    //     .recursive(true)
    //     .create(path)?;

    // 4. Create a few files for the next step.
    // `write` creates a file if it does not exist, and will entirely replace
    // its contents if it does.
    fs::write(base.join("temp/my_files/file1.txt"), "Hello")?;
    fs::write(base.join("temp/my_files/subfolder/file2.log"), "World")?;

    // 5. List the contents of a directory:
    println!("Reading contents of 'my_files':");
    // `read_dir` returns an iterator over the directory's entries.
    // Note that the order in which `read_dir` returns entries is not
    // guaranteed.
    // Use recursion to list the contents of all (sub)directories.
    for entry in fs::read_dir(base.join("temp/my_files"))? {
        let entry = entry?; // Handle potential error for each entry.
        let path = entry.path();

        if path.is_dir() {
            println!("- (Directory) {}", path.display());
        } else {
            println!("- (File)      {}", path.display());
        }
    }

    // 6. `std::fs::remove_dir_all` removes a directory at a given path, after
    // removing all its contents - similar to `rm -rf` on Unix.
    // Note that `remove_dir_all` returns an error if the target directory does
    // not exist or removing any of its contents fails. This function does not
    // follow symbolic links and
    // will simply remove the symbolic link itself.
    fs::remove_dir_all(base.join("temp/my_files"))?;
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> std::io::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the manipulate dirs example through its entry point.
    #[test]
    fn test_manipulate_dirs_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// Everything is created and cleaned up under the given base directory.
    #[test]
    fn creates_and_removes_directories_in_base() -> std::io::Result<()> {
        let base = tempfile::tempdir()?;
        manipulate_dirs(base.path())?;
        assert!(!base.path().join("example_dir").exists());
        assert!(!base.path().join("temp/my_files").exists());
        assert!(base.path().join("temp").is_dir());
        Ok(())
    }

    /// A pre-existing `example_dir` is reused; a non-empty one cannot be
    /// removed.
    #[test]
    fn non_empty_example_dir_is_an_error() -> std::io::Result<()> {
        let base = tempfile::tempdir()?;
        fs::create_dir(base.path().join("example_dir"))?;
        manipulate_dirs(base.path())?;
        fs::create_dir(base.path().join("example_dir"))?;
        fs::write(base.path().join("example_dir/f"), "x")?;
        assert!(manipulate_dirs(base.path()).is_err());
        Ok(())
    }
}
