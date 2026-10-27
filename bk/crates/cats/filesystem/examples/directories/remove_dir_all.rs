#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Add to `Cargo.toml`:
//! ```toml
//! remove_dir_all = { version = "1.0.0", features = [ "parallel" ] }
//! ```

use std::io;

use remove_dir_all::remove_dir_all;

/// Creates a directory tree, then removes it with all its content.
fn main() -> io::Result<()> {
    // Create a directory structure for demonstration purposes
    // (if it doesn't exist already).
    let path = "temp/example_dir/sub_dir";
    std::fs::create_dir_all(path)?;
    std::fs::write("temp/example_dir/file1.txt", "Hello, world!")?;
    std::fs::write("temp/example_dir/sub_dir/file2.txt", "Rust is awesome!")?;

    // Now, remove the subdirectory and all of its contents.
    match remove_dir_all(path) {
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
}
