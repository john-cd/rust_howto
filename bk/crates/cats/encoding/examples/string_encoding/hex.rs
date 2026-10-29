// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Example of encoding and decoding a string using hexadecimal encoding.

use data_encoding::DecodeError;
use data_encoding::HEXUPPER;

/// Encodes bytes as hexadecimal, then decodes them.
fn main() -> Result<(), DecodeError> {
    // The original string to be encoded.
    let original = b"The quick brown fox jumps over the lazy dog.";
    let expected = "54686520717569636B2062726F776E20666F78206A756D7073206F76\
        657220746865206C617A7920646F672E";

    let encoded = HEXUPPER.encode(original);
    println!("{encoded}");
    assert_eq!(encoded, expected);

    let decoded = HEXUPPER.decode(&encoded.into_bytes())?;
    println!("{decoded:?}");
    assert_eq!(&*decoded, &original[..]);

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<(), DecodeError> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the hex example through its entry point.
    #[test]
    fn test_hex_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// Lowercase and odd-length input is rejected by the uppercase decoder.
    #[test]
    fn rejects_invalid_hex() {
        assert_eq!(HEXUPPER.encode(b"\x00\xff"), "00FF");
        assert!(HEXUPPER.decode(b"abc").is_err());
        assert!(HEXUPPER.decode(b"zz").is_err());
        assert_eq!(HEXUPPER.decode(b"").unwrap(), Vec::<u8>::new());
    }
}
