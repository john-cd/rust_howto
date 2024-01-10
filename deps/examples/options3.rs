#![allow(clippy::bind_instead_of_map)]

use std::fs;

// `and_then` applies a function to the wrapped value if it's Some.
fn read_file(filename: &str) -> Option<String> {
    fs::read_to_string(filename)
        .ok() // Convert `Result` to `Option`
        .and_then(|contents| Some(contents.trim().to_string()))
    // `and_then` chains operations on `Some`
}

fn main() {
    let contents = read_file("poem.txt");

    // Using `match` to process the returned Option.
    match contents {
        Some(poem) => println!("{}", poem),
        None => println!("Error reading file"),
    }
}
