#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Find duplicate filenames in the current directory.

use std::collections::HashMap;
use std::ffi::OsString;

use anyhow::Result;
use walkdir::WalkDir;

/// Prints the file names that appear more than once in the current directory
/// tree.
fn main() -> Result<()> {
    let mut filenames = HashMap::<OsString, usize>::new();

    for entry in WalkDir::new(".") {
        let entry = entry?;
        if entry.file_type().is_dir() {
            continue;
        }

        let f_name = entry.file_name().to_os_string();
        let counter = filenames.entry(f_name.clone()).or_insert(0);
        *counter += 1;

        if *counter == 2 {
            println!("{}", f_name.to_string_lossy());
        }
    }
    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the example can traverse the current directory.
    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
