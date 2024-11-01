// ANCHOR: example
use std::env;

use anyhow::Result;

fn main() -> Result<()> {
    // Load environment variables from .env file.
    // Fails if .env file not found, not readable or invalid.
    dotenvy::dotenv()?;

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
