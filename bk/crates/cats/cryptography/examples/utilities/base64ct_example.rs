// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Encode a fixed-size token with `base64ct`.
//! Base64 is a text representation, not encryption. This crate's decoder
//! avoids data-dependent lookup tables for sensitive byte sequences.

use base64ct::Base64;
use base64ct::Encoding;

/// Handles the round trip behavior for this component.
fn round_trip(token: &[u8]) -> Result<Vec<u8>, base64ct::Error> {
    let encoded = Base64::encode_string(token);
    Base64::decode_vec(&encoded)
}

/// Runs the example demonstrated by this module.
fn main() -> anyhow::Result<()> {
    let token = b"example-token-123";
    let decoded = round_trip(token)?;
    assert_eq!(decoded, token);
    println!("{}", Base64::encode_string(token));
    Ok(())
}
// ANCHOR_END: example

/// Runs the example logic exposed by this module.
pub fn run() -> anyhow::Result<()> {
    main()
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that token round trips.
    #[test]
    fn token_round_trips() -> anyhow::Result<()> {
        assert_eq!(round_trip(b"\0secret\xff")?, b"\0secret\xff");
        Ok(())
    }
}
