// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Encode and decode bytes with the `base64` crate.

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;

/// Encodes a byte string as Base64 and decodes it again.
fn main() -> anyhow::Result<()> {
    let message = b"Rust cryptography utilities";
    let encoded = STANDARD.encode(message);
    let decoded = STANDARD.decode(&encoded)?;

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
