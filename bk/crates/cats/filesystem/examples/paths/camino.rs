#![allow(dead_code)]
// ANCHOR: example
use std::path::PathBuf;

use camino::Utf8PathBuf;

fn main() {
    // The standard `PathBuf` can contain non-UTF-8 data:
    let mut path = PathBuf::from("/home/user");
    path.push("résumé.pdf"); // This might be non-UTF-8 on some systems.

    // .to_str() returns an Option, which can be `None`.
    match path.to_str() {
        Some(s) => println!("The path is: {}", s),
        None => println!("Path is not valid UTF-8."),
    }

    // `Utf8Path` and `Utf8PathBuf` work just like their standard library
    // counterparts but guarantee their content is valid UTF-8, eliminating
    // the need to check.

    // Create a new Utf8PathBuf. The `from` method works like the standard one.
    let mut path = Utf8PathBuf::from("data/files/résumé");
    println!("Initial path: {}", path);

    // Push a new component onto the path.
    // This is guaranteed to be UTF-8.
    path.push("résumé-2025.txt");
    println!("Final path: {}", path);

    // You can directly use the path as a `&str` without any `to_str()` call.
    if path.as_str().ends_with(".txt") {
        println!("This is a text file.");
    }

    // You can also get e.g. the file name directly:
    if let Some(file_name) = path.file_name() {
        println!("File name is: {}", file_name);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
