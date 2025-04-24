// ANCHOR: example
use std::fs::File;

/// `main` returning a `Result`, in this case a specialized `Result` type for
/// I/O operations.
fn main() -> std::io::Result<()> {
    let _f = File::open("non_existent_file.txt")?; // The ? operator propagates the error.
    println!("File opened successfully (this won't print).");
    // Would return `Ok(())` on success.
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() {
    assert!(main().is_err());
}
