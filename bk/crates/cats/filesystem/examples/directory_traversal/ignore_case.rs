#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Demonstrates how to ignore case when matching file names.

use anyhow::Result;
use glob::MatchOptions;
use glob::glob_with;

/// Prints the paths that match a glob pattern, ignoring case.
fn main() -> Result<()> {
    let options = MatchOptions {
        case_sensitive: false,
        ..Default::default()
    };

    for entry in glob_with("/media/img_[0-9]*.png", options)? {
        println!("{}", entry?.display());
    }

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the ignore case example through its entry point.
    #[test]
    fn test_ignore_case_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
