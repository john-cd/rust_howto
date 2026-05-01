#![allow(dead_code)]
#![allow(clippy::bind_instead_of_map)]
#![allow(clippy::unwrap_or_default)]
// ANCHOR: example
use std::fs;

/// Reads a file and returns its contents as `Some(String)` containing the
/// trimmed, lower-case file contents if successful, otherwise `None`.
fn read_file(filename: &str) -> Option<String> {
    fs::read_to_string(filename)
        // Convert a `Result` to `Option`.
        .ok()
        // `and_then` applies a function to the wrapped value if it's `Some`.
        .and_then(|contents| Some(contents.trim().to_string()))
        // `map` is similar, but the closure does not need to return an `Option`:
        .map(|s| s.to_lowercase())
}

fn main() {
    let contents_maybe = read_file("temp/poem.txt");
    // Provide a default with `unwrap_or_else` or `unwrap_or_default`.
    let contents = contents_maybe.unwrap_or_else(String::new);
    println!("{contents}");
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    if !std::fs::exists("temp")? {
        std::fs::create_dir("temp")?;
    }
    fs::write("temp/poem.txt", b" Lorem ipsum ")?;
    main();
    Ok(())
}
