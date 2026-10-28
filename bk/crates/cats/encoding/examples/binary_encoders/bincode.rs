// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Serialize and deserialize a struct with `bincode` 2.x.
//!
//! Version 2 changed the API: encode with `bincode::serde::encode_to_vec` and
//! decode with `bincode::serde::decode_from_slice`, using the same explicit
//! configuration on both sides. These replace the familiar `serialize` and
//! `deserialize` functions from version 1. This example uses the 2.0.1 release;
//! the upstream repository is archived, so evaluate maintained alternatives
//! before choosing a format for a new system.

use bincode::config::standard;
use bincode::serde::decode_from_slice;
use bincode::serde::encode_to_vec;
use serde::Deserialize;
use serde::Serialize;

/// Represents a struct that can be serialized and deserialized
/// using bincode.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct MyData {
    /// The name.
    name: String,
    /// The age in years.
    age: u32,
    /// Whether the data is active.
    is_active: bool,
}

/// Encodes a struct with bincode 2, then decodes it with the same
/// configuration.
///
/// # Errors
///
/// Returns an error if serialization or deserialization fails.
fn main() -> anyhow::Result<()> {
    let data = MyData {
        name: "Alice".to_owned(),
        age: 30,
        is_active: true,
    };

    let encoded = encode_to_vec(&data, standard())?;
    let (decoded, consumed): (MyData, usize) =
        decode_from_slice(&encoded, standard())?;
    assert_eq!(data, decoded);
    assert_eq!(encoded.len(), consumed);

    println!(
        "Encoded {} bytes; decoder consumed {consumed} bytes",
        encoded.len()
    );
    println!("Decoded value: {decoded:?}");
    Ok(())
}
// ANCHOR_END: example

/// # Errors
///
/// Returns an error if serialization or deserialization fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The sample value used by the tests below.
    fn sample() -> MyData {
        MyData {
            name: "Alice".to_owned(),
            age: 30,
            is_active: true,
        }
    }

    /// The example's end-to-end encode and decode path succeeds.
    /// Runs the bincode example through its entry point.
    #[test]
    fn test_bincode_example_entry_point() -> anyhow::Result<()> {
        main()
    }

    /// Pins the wire format: a varint length, the UTF-8 bytes, a varint age
    /// and one byte for the boolean.
    #[test]
    fn standard_config_uses_varint_integers() -> anyhow::Result<()> {
        let encoded = encode_to_vec(sample(), standard())?;
        assert_eq!(
            encoded,
            [5, b'A', b'l', b'i', b'c', b'e', 30, 1],
            "unexpected bincode wire format"
        );
        Ok(())
    }

    /// Both sides of a connection must agree on the configuration.
    #[test]
    fn fixed_int_encoding_is_larger_and_incompatible_with_standard()
    -> anyhow::Result<()> {
        let fixed = bincode::config::standard().with_fixed_int_encoding();
        let encoded = encode_to_vec(sample(), fixed)?;
        // 8-byte length + 5 name bytes + 4-byte age + 1 boolean byte.
        assert_eq!(encoded.len(), 18);

        let (decoded, _): (MyData, usize) = decode_from_slice(&encoded, fixed)?;
        assert_eq!(decoded, sample());

        // Decoding with the wrong configuration does not yield the value.
        let wrong = decode_from_slice::<MyData, _>(&encoded, standard());
        assert!(wrong.map_or(true, |(value, _)| value != sample()));
        Ok(())
    }

    /// Truncated bytes are rejected by the decoder.
    #[test]
    fn truncated_input_is_an_error() -> anyhow::Result<()> {
        let encoded = encode_to_vec(sample(), standard())?;
        let truncated = &encoded[..encoded.len() - 2];
        assert!(decode_from_slice::<MyData, _>(truncated, standard()).is_err());
        Ok(())
    }
}
