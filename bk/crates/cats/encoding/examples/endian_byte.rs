// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Encoding and decoding a struct in little-endian byte order with `byteorder`.
//!
//! Endianness is the order in which the bytes of a multi-byte number are
//! stored. Little-endian puts the least significant byte first (x86 and ARM
//! computers); big-endian puts the most significant first (network protocols).
//! Files and network formats must fix one order so that every machine reads the
//! same value.

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

    /// Runs the endian byte example through its entry point.
    #[test]
    fn test_endian_byte_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// Encodes little-endian, round-trips, and rejects short input.
    #[test]
    fn encodes_little_endian_and_rejects_short_input() -> Result<(), Error> {
        let payload = Payload {
            kind: 7,
            value: 0x0102,
        };
        let bytes = encode(&payload)?;
        assert_eq!(bytes, vec![7, 0x02, 0x01]);
        assert_eq!(decode(&bytes)?, payload);

        assert!(decode(&[]).is_err());
        assert!(decode(&[7, 0x02]).is_err());
        Ok(())
    }
}
