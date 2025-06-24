#![allow(dead_code)]
// ANCHOR: example
use std::env;

use anyhow::Result;

/// Load environment variables from a `.env` file in the current directory.
fn main() -> Result<()> {
    // Load environment variables from .env file.
    // Fails if .env file not found, not readable or invalid.
    // If variables with the same names already exist in the environment,
    // their values will be preserved. If multiple declarations for the
    // same environment variable exist in your .env file, the first one is
    // applied.
    dotenvy::dotenv()?;
    // OR: dotenvy::dotenv().ok();

    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> Result<()> {
    main()?;
    Ok(())
}
