#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to encode and decode a string using `base64`.

use std::str;

use anyhow::Result;
use base64::prelude::*;

/// Encodes bytes as Base64, then decodes them.
fn main() -> Result<()> {
    let hello = b"hello rustaceans";
    let encoded: String = BASE64_STANDARD.encode(hello);
    let decoded: Vec<u8> = BASE64_STANDARD.decode(&encoded)?;

    println!("origin: {}", str::from_utf8(hello)?);
    println!("base64 encoded: {encoded}");
    println!("back to origin: {}", str::from_utf8(&decoded)?);

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the base64 example through its entry point.
    #[test]
    fn test_base64_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
