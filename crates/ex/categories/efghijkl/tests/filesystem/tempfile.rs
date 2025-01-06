// ANCHOR: example
// src/main.rs
use std::io::Write;
use std::io::{self};

use tempfile::NamedTempFile;

// Create a named temporary file, write to it, read from it, and print its path and content using the tempfile crate in Rust.

fn main() -> io::Result<()> {
    // Create a named temporary file
    let mut temp_file = NamedTempFile::new()?;

    // Write some data to the temporary file
    writeln!(temp_file, "Hello, temporary file!")?;

    // Print the path of the temporary file
    println!("Temporary file created at: {:?}", temp_file.path());

    // Read the content back from the temporary file
    let mut file = temp_file.reopen()?;
    let mut content = String::new();
    std::io::Read::read_to_string(&mut file, &mut content)?;

    // Print the content
    println!("Content of temporary file: {}", content);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [P0](https://github.com/john-cd/rust_howto/issues/763)
