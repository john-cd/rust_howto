#![allow(clippy::bind_instead_of_map)]
// ANCHOR: example

use std::fs;

fn read_file(filename: &str) -> Option<String> {
    fs::read_to_string(filename)
        // Convert `Result` to `Option`
        .ok()
        // `and_then` applies a function to the wrapped value if it's Some.
        .and_then(|contents| Some(contents.trim().to_string()))
}

fn main() -> anyhow::Result<()> {
    if !std::fs::exists("temp")? {
        std::fs::create_dir("temp")?;
    }
    fs::write("temp/poem.txt", b"Lorem ipsum")?;
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
fn test() {
    main().unwrap();
}
