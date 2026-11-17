// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Serialize and deserialize a struct with `bincode` 1.3.3.
//!
//! Bincode 1.3.3 provides the familiar `serialize` and `deserialize` helpers.
//! The project considers this release complete; Bincode 2 is unmaintained, so
//! evaluate actively maintained alternatives before choosing a format for a
//! new system.

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

/// Encodes and decodes a struct with Bincode's default configuration.
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

    let encoded = bincode::serialize(&data)?;
    let decoded: MyData = bincode::deserialize(&encoded)?;
    assert_eq!(data, decoded);

    println!("Encoded {} bytes", encoded.len());
    println!("Decoded value: {decoded:?}");
    Ok(())
}
// ANCHOR_END: example

/// Serializes and deserializes the example data with Bincode.
///
/// # Errors
///
/// Returns an error if serialization or deserialization fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

/// The example's end-to-end encode and decode path succeeds.
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
    #[test]
    fn test_bincode_example_entry_point() -> anyhow::Result<()> {
        main()
    }

    /// Checks that the default helper uses fixed-width little-endian integers.
    #[test]
    fn default_helper_uses_fixed_width_little_endian_integers() -> anyhow::Result<()> {
        let encoded = bincode::serialize(&sample())?;
        assert_eq!(
            encoded,
            [5, 0, 0, 0, 0, 0, 0, 0, b'A', b'l', b'i', b'c', b'e', 30, 0, 0, 0, 1],
            "unexpected bincode wire format"
        );
        Ok(())
    }

    /// Both sides of a connection must agree on the encoded type and format.
    #[test]
    fn serialized_data_round_trips() -> anyhow::Result<()> {
        let encoded = bincode::serialize(&sample())?;
        let decoded: MyData = bincode::deserialize(&encoded)?;
        assert_eq!(decoded, sample());
        Ok(())
    }

    /// Truncated bytes are rejected by the decoder.
    #[test]
    fn truncated_input_is_an_error() -> anyhow::Result<()> {
        let encoded = bincode::serialize(&sample())?;
        let truncated = &encoded[..encoded.len() - 2];
        assert!(bincode::deserialize::<MyData>(truncated).is_err());
        Ok(())
    }
}
