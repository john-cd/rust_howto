// ANCHOR: example
//! Demonstrates the use of the `remove_dir_all` crate to recursively delete a
//! directory and its contents.
//!
//! The `remove_dir_all` library provides an alternative implementation of
//! `std::fs::remove_dir_all` from the Rust std library.
//!
//! Add to `Cargo.toml`:
//! ```toml
//! remove_dir_all = { version = "1.0.0", features = [ "parallel" ] }
//! ```
//!
//! The optional 'parallel' feature parallelises the deletion. This is useful
//! when high syscall latency is occurring, such as on Windows or network file
//! systems.

use std::io;

use remove_dir_all::remove_dir_all;

fn main() -> io::Result<()> {
    // Create a directory structure for demonstration purposes
    // (if it doesn't exist already).
    let path = "temp/example_dir/sub_dir";
    if let Err(e) = std::fs::create_dir_all(path) {
        println!("Error: {}", e);
    }
    std::fs::write("temp/example_dir/file1.txt", "Hello, world!")?;
    std::fs::write("temp/example_dir/sub_dir/file2.txt", "Rust is awesome!")?;

    // Now, remove the subdirectory and all of its contents.
    match remove_dir_all(path) {
        Ok(_) => println!(
            "Directory 'sub_dir' and all contents removed successfully!"
        ),
        Err(e) => eprintln!("Failed to remove directory: {}", e),
    }
    // Beware:
    // - Calling this on a non-directory (e.g. a symlink to a directory)
    // will error.
    // - It assumes both that the caller has permission to delete
    // the files, and that they don’t have permission to change permissions to
    // be able to delete the files: no ACL or chmod changes are made during
    // deletion.

    // Security note: The functions `remove_dir_all`, `remove_dir_contents`, and
    // `ensure_empty_dir` are intrinsically sensitive to file system races.
    // The crate authors recommend using the `RemoveDir` trait instead.
    // This allows callers to be more confident that what is deleted is what was
    // requested even in the presence of (malicious) actors changing the
    // filesystem concurrently.
    let mut dir = std::fs::File::open("temp/example_dir")?;
    use remove_dir_all::RemoveDir;
    dir.remove_dir_contents(None)?;
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> io::Result<()> {
    main()?;
    Ok(())
}
