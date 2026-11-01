// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Encode and decode bytes as hexadecimal with the `hex` crate.
//!
//! Hexadecimal writes each byte as two characters from 0-9 and a-f. It is the
//! usual way to display hashes, keys and binary data. It doubles the size of
//! the data and is not encryption.

/// Encodes a byte string as hexadecimal and decodes it again.
fn main() -> anyhow::Result<()> {
    let message = b"Rust cryptography utilities";
    let encoded = hex::encode(message);
    let decoded = hex::decode(&encoded)?;

    assert_eq!(decoded, message);
    println!("{encoded}");
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the example can encode and decode its input.
    /// Runs the hex example example through its entry point.
    #[test]
    fn test_hex_example_example_entry_point() -> anyhow::Result<()> {
        main()
    }
}
