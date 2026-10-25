// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Encode and decode bytes as hexadecimal with the `hex` crate.

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

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the example can encode and decode its input.
    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }
}
