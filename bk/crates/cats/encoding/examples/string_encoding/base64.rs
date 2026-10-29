// TODO Reviewed ??-??-2026.

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

    /// Known vectors encode as expected; invalid input is rejected.
    #[test]
    fn encodes_known_vectors_and_rejects_invalid_input() -> Result<()> {
        assert_eq!(
            BASE64_STANDARD.encode(b"hello rustaceans"),
            "aGVsbG8gcnVzdGFjZWFucw=="
        );
        assert_eq!(BASE64_STANDARD.encode(b""), "");
        assert_eq!(BASE64_STANDARD.decode("aGk=")?, b"hi");
        assert!(BASE64_STANDARD.decode("not base64!").is_err());
        Ok(())
    }
}
