// ANCHOR: example
use ignore::WalkBuilder;

// The `ignore` crate provides a fast recursive directory iterator
// that respects `.gitignore` and similar files, ignores hidden files, etc.

// In `Cargo.toml`, add:
// ignore = "0.4" # or latest

fn main() {
    // Specify the directory to walk
    let root = std::path::Path::new("tests/");

    // Create a WalkBuilder and build a new Walk iterator.
    // A lot more options are available
    let walker = WalkBuilder::new(root)
        .add("temp/") // Add another directory to walk
        .standard_filters(true)  // Respect .gitignore and other ignore files
        .build();

    // Iterate over the entries
    for result in walker {
        match result {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() {
                    println!("File: {:?}", path);
                } else if path.is_dir() {
                    println!("Directory: {:?}", path);
                }
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
