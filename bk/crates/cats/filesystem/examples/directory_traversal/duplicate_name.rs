#![allow(dead_code)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Find duplicate filenames in the current directory.

use std::collections::HashMap;

use walkdir::WalkDir;

/// Prints the file names that appear more than once in the current directory
/// tree.
fn main() {
    let mut filenames = HashMap::new();

    for entry in WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        let counter = filenames.entry(f_name.clone()).or_insert(0);
        *counter += 1;

        if *counter == 2 {
            println!("{f_name}");
        }
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
