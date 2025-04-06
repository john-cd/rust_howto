// ANCHOR: example
//! Demonstrates the use of the `remove_dir_all` crate to recursively delete a
//! directory and its contents.
//!
//! The `remove_dir_all` library provides an alternative implementation of
//! `std::fs::remove_dir_all` from the Rust std library.

use std::io;

use remove_dir_all::remove_dir_all;

fn main() -> io::Result<()> {
    // Create a directory structure for demonstration purposes
    // (if it doesn't exist already).
    std::fs::create_dir_all("temp/example_dir/sub_dir")?;
    std::fs::write("temp/example_dir/file1.txt", "Hello, world!")?;
    std::fs::write("temp/example_dir/sub_dir/file2.txt", "Rust is awesome!")?;

    println!("Directory and files created successfully!");

    // Now, remove the directory and all of its contents.
    match remove_dir_all("temp/example_dir") {
        Ok(_) => println!(
            "Directory 'example_dir' and all contents removed successfully!"
        ),
        Err(e) => eprintln!("Failed to remove directory 'example_dir': {}", e),
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> io::Result<()> {
    main()?;
    Ok(())
}
// [review NOW](https://github.com/john-cd/rust_howto/issues/1005)
