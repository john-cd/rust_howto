#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to traverse a directory and find files that
//! have been modified within the last 24 hours.

use anyhow::Result;
use walkdir::WalkDir;

fn main() -> Result<()> {
    // Traverse the current directory recursively.
    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();

        // `metadata()` can return errors for path values that the program
        // does not have permissions to access or if the path no longer exists.
        if let Ok(metadata) = entry.metadata() {
            // Get the last modified time of the file.
            let sec = metadata.modified()?;
            // Calculate the elapsed time since the last modification.
            if let Ok(elapsed) = sec.elapsed() {
                // Check if the file was modified within the last 24 hours
                // (86400 seconds).
                if elapsed.as_secs() < 86400 {
                    // Print the file name if it was modified recently.
                    println!("{f_name}");
                }
            }
        }
        // You may also check for specific extensions by adding:
        // && f_name.ends_with(".json")
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// TODO review
