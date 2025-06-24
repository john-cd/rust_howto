#![allow(dead_code)]
// ANCHOR: example
use walkdir::WalkDir;

/// Calculates the total size of all files within a directory tree (within a
/// given depth).
fn main() {
    // Create a new WalkDir iterator starting from the current directory (".").
    let total_size = WalkDir::new(".")
        .min_depth(1) // Set the minimum depth to 1 (exclude the root directory itself).
        .max_depth(3)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .fold(0, |acc, m| acc + m.len());

    println!("Total size: {} bytes.", total_size);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
