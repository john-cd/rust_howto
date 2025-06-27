#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to traverse a directory and check the last
//! modified time of each file.

use std::env;
use std::fs;

use anyhow::Result;
use anyhow::anyhow;

/// Checks whether the files in the current directory were modified in the last
/// 24 hours.
fn main() -> Result<()> {
    // Get the current directory.
    let current_dir = env::current_dir()?;
    println!("Entries modified in the last 24 hours in {current_dir:?}:");

    // Iterate over the entries within a directory.
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::metadata(&path)?;
        if let Ok(time) = metadata.modified() {
            // Returns the difference from the file's last modification time to
            // the current clock time. Note: SystemTime.elapsed can
            // be flaky.
            if let Ok(duration) = time.elapsed() {
                let last_modified = duration.as_secs();
                if (last_modified < 24 * 3600) && metadata.is_file() {
                    println!(
                        "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                        last_modified,
                        metadata.permissions().readonly(),
                        metadata.len(),
                        path.file_name().ok_or(anyhow!("No filename."))?
                    );
                }
            }
        } else {
            println!(
                "Last modification time is not supported on this platform."
            );
        }
    }
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [review flaky test(https://github.com/john-cd/rust_howto/issues/165)
