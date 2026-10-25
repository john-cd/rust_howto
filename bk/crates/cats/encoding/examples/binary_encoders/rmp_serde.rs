#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Serialize and deserialize a struct with MessagePack via `rmp-serde`.
//!
//! `to_vec_named` encodes a struct as a map with field names. The compact
//! `to_vec` form encodes struct values as an array and relies on field order.

use serde::Deserialize;
use serde::Serialize;

/// A struct that is serialized to and deserialized from MessagePack.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct MyData {
    /// The name.
    name: String,
    /// The age in years.
    age: u32,
    /// Whether the data is active.
    is_active: bool,
}

/// Encodes a struct as MessagePack, then decodes it.
fn main() -> anyhow::Result<()> {
    let data = MyData {
        name: "Alice".to_owned(),
        age: 30,
        is_active: true,
    };

    let encoded = rmp_serde::to_vec_named(&data)?;
    let decoded: MyData = rmp_serde::from_slice(&encoded)?;
    assert_eq!(data, decoded);

    println!("Encoded {} bytes", encoded.len());
    println!("Decoded value: {decoded:?}");
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

    /// The sample value used by the tests below.
    fn sample() -> MyData {
        MyData {
            name: "Alice".to_owned(),
            age: 30,
            is_active: true,
        }
    }

    #[test]
    fn named_messagepack_round_trip_preserves_the_struct() -> anyhow::Result<()>
    {
        main()
    }

    /// `to_vec` writes a positional array, which is smaller than the map
    /// written by `to_vec_named` but depends on the field order.
    #[test]
    fn compact_encoding_is_an_array_and_smaller_than_named()
    -> anyhow::Result<()> {
        let compact = rmp_serde::to_vec(&sample())?;
        let named = rmp_serde::to_vec_named(&sample())?;
        assert!(compact.len() < named.len());
        // 0x93 is a fixarray of length 3; 0x83 is a fixmap of length 3.
        assert_eq!(compact.first(), Some(&0x93));
        assert_eq!(named.first(), Some(&0x83));

        let decoded: MyData = rmp_serde::from_slice(&compact)?;
        assert_eq!(decoded, sample());
        Ok(())
    }

    /// The named form is self-describing: it can be read as a generic value.
    #[test]
    fn named_encoding_exposes_field_names() -> anyhow::Result<()> {
        let encoded = rmp_serde::to_vec_named(&sample())?;
        let value: serde_json::Value = rmp_serde::from_slice(&encoded)?;
        assert_eq!(value["name"], "Alice");
        assert_eq!(value["age"], 30);
        assert_eq!(value["is_active"], true);
        Ok(())
    }

    #[test]
    fn truncated_input_is_an_error() -> anyhow::Result<()> {
        let encoded = rmp_serde::to_vec_named(&sample())?;
        let truncated = &encoded[..encoded.len() - 2];
        assert!(rmp_serde::from_slice::<MyData>(truncated).is_err());
        Ok(())
    }
}
