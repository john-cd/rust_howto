#![allow(dead_code)]
// ANCHOR: example
use std::fs::File;

/// Example of a `main` function returning a `Result`,
/// in this case a specialized `Result` type for I/O operations.
fn main() -> std::io::Result<()> {
    let _f = File::open("non_existent_file.txt")?; // The ? operator returns early, if there is an error.
    println!("File opened successfully (this won't print).");
    // Would return `Ok(())` on success.
    // The inner parens represent the unit type, i.e. the absence of a
    // meaningful return value.
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() {
    assert!(main().is_err());
}
