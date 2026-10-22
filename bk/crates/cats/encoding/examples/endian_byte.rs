//! Encoding and decoding a struct in little-endian byte order with `byteorder`.
//!
//! TODO Reviewed ??-??-2026

// ANCHOR: example
use std::io::Error;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;

// Define a struct `Payload` to represent the data we want to encode/decode.
/// The data to encode.
#[derive(Default, PartialEq, Debug)]
struct Payload {
    /// Kind of payload.
    kind: u8,
    /// Value of the payload.
    value: u16,
}

/// Convert a Payload into a vector of bytes.
fn encode(payload: &Payload) -> Result<Vec<u8>, Error> {
    let mut bytes = vec![];
    // Writes an unsigned 8 bit integer.
    bytes.write_u8(payload.kind)?;
    // Writes an unsigned 16 bit integer.
    bytes.write_u16::<LittleEndian>(payload.value)?;
    Ok(bytes)
}

/// Read the bytes from the slice and construct a `Payload` instance.
fn decode(mut bytes: &[u8]) -> Result<Payload, Error> {
    let payload = Payload {
        kind: bytes.read_u8()?,
        value: bytes.read_u16::<LittleEndian>()?,
    };
    Ok(payload)
}

/// Encodes a payload, decodes it and compares the two.
fn main() -> Result<(), Error> {
    // Create a default `Payload` instance.
    let original_payload = Payload::default();

    // Encode the payload into a byte vector.
    let encoded_bytes = encode(&original_payload)?;
    println!("{encoded_bytes:?}");

    // Decode the byte vector back into a `Payload` instance.
    let decoded_payload = decode(&encoded_bytes)?;

    // Assert that the original and decoded payloads are equal.
    assert_eq!(original_payload, decoded_payload);
    println!("{decoded_payload:?}");
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
