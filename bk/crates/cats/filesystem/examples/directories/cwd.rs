#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Demonstrates how to get the current working directory.

use std::env;
use std::path::PathBuf;

use anyhow::Result;

/// Prints the current working directory.
fn main() -> Result<()> {
    let cwd: PathBuf = env::current_dir()?;
    println!("The current directory is {}", cwd.display());
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

    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
