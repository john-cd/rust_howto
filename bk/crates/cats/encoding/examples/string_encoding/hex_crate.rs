// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Encode and decode hexadecimal with the `hex` crate.

/// Encodes bytes as lowercase hexadecimal and decodes them back.
pub fn run() -> Result<(), hex::FromHexError> {
    let encoded = hex::encode("Hello world!");
    assert_eq!(encoded, "48656c6c6f20776f726c6421");

    let decoded = hex::decode(&encoded)?;
    assert_eq!(decoded, b"Hello world!");

    // Invalid input is reported, not ignored.
    assert_eq!(hex::decode("123"), Err(hex::FromHexError::OddLength));
    assert_eq!(
        hex::decode("zz"),
        Err(hex::FromHexError::InvalidHexCharacter { c: 'z', index: 0 })
    );

    // Decode into a fixed-size array.
    let mut bytes = [0_u8; 2];
    hex::decode_to_slice("cafe", &mut bytes)?;
    assert_eq!(bytes, [0xca, 0xfe]);
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_succeeds() -> Result<(), hex::FromHexError> {
        run()
    }
}
