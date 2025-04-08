#![allow(clippy::bind_instead_of_map)]
// ANCHOR: example
use std::fs;

/// Reads a file and returns its contents as a trimmed string.
///
/// # Arguments
///
/// * `filename` - The name of the file to read.
///
/// # Returns
///
/// Returns `Some(String)` containing the trimmed file contents if successful,
/// otherwise `None`.
fn read_file(filename: &str) -> Option<String> {
    fs::read_to_string(filename)
        // Convert `Result` to `Option`.
        .ok()
        // `and_then` applies a function to the wrapped value if it's `Some`.
        .and_then(|contents| Some(contents.trim().to_string()))
}

fn main() -> anyhow::Result<()> {
    let contents = read_file("temp/poem.txt");

    // Using `match` to process the returned Option.
    match contents {
        Some(poem) => println!("{}", poem),
        None => println!("Error reading file"),
    }
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    if !std::fs::exists("temp")? {
        std::fs::create_dir("temp")?;
    }
    fs::write("temp/poem.txt", b"Lorem ipsum")?;
    main()?;
    Ok(())
}
