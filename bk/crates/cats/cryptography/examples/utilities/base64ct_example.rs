// TODO Reviewed ??-??-2026.

// ANCHOR: example

//! Encode a fixed-size token with `base64ct`.
//! Base64 is a text representation, not encryption. This crate's decoder
//! avoids data-dependent lookup tables for sensitive byte sequences.

use base64ct::Base64;
use base64ct::Encoding;

fn round_trip(token: &[u8]) -> Result<Vec<u8>, base64ct::Error> {
    let encoded = Base64::encode_string(token);
    Base64::decode_vec(&encoded)
}

fn main() -> anyhow::Result<()> {
    let token = b"example-token-123";
    let decoded = round_trip(token)?;
    assert_eq!(decoded, token);
    println!("{}", Base64::encode_string(token));
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_round_trips() -> anyhow::Result<()> {
        assert_eq!(round_trip(b"\0secret\xff")?, b"\0secret\xff");
        Ok(())
    }
}
