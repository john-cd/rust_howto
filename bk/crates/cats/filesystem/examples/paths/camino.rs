#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! UTF-8 paths with `camino`.

use std::path::PathBuf;

use camino::Utf8PathBuf;

/// Compares standard paths with UTF-8 paths.
fn main() {
    // The standard `PathBuf` can contain non-UTF-8 data:
    let mut path = PathBuf::from("/home/user");
    path.push("résumé.pdf"); // This component is UTF-8 because it comes from a Rust string.

    // A `PathBuf` can hold non-UTF-8 data when it comes from the operating
    // system, but this path was built only from UTF-8 string components.
    if let Some(s) = path.to_str() {
        println!("The path is: {s}");
    }

    // `Utf8Path` and `Utf8PathBuf` work just like their standard library
    // counterparts but guarantee their content is valid UTF-8, eliminating
    // the need to check.

    // Create a new Utf8PathBuf. The `from` method works like the standard one.
    let mut path = Utf8PathBuf::from("data/files/résumé");
    println!("Initial path: {path}");

    // Push a new component onto the path.
    // This is guaranteed to be UTF-8.
    path.push("résumé-2025.txt");
    println!("Final path: {path}");

    // You can directly use the path as a `&str` without any `to_str()` call.
    if path.as_str().ends_with(".txt") {
        println!("This is a text file.");
    }

    // You can also get e.g. the file name directly:
    if let Some(file_name) = path.file_name() {
        println!("File name is: {file_name}");
    }
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
