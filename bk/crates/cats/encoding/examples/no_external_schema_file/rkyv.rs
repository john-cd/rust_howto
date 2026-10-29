// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! `rkyv` (short for "archive") is a high-performance, zero-copy
//! (de)serialization framework that allows arbitrary field types
//! and safe zero-copy mutation.
//!
//! This example demonstrates how to serialize and deserialize a struct
//! using `rkyv` without an external schema file.

use rkyv::Archive;
use rkyv::Deserialize;
use rkyv::Serialize;
use rkyv::rancor::Error;

/// A struct that can be archived.
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Eq)]
struct MyStruct {
    /// A string.
    field1: String,
    /// A number.
    field2: i32,
}

/// Serializes a struct with `rkyv`, then deserializes it.
fn main() -> anyhow::Result<()> {
    // Create an instance of `MyStruct`.
    let data = MyStruct {
        field1: "Hello, world!".to_owned(),
        field2: 42,
    };

    // Serialize the data to a byte array.
    let buffer = rkyv::to_bytes::<Error>(&data)?;

    // Print the serialized data.
    println!("Serialized data: {buffer:?}");

    // Deserialize the data from the byte array.
    let deserialized_data = rkyv::from_bytes::<MyStruct, Error>(&buffer)?;

    println!("{deserialized_data:?}");

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the rkyv example through its entry point.
    #[test]
    fn test_rkyv_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// Values round-trip, and malformed bytes are rejected by validation.
    #[test]
    fn roundtrips_and_rejects_invalid_bytes() -> anyhow::Result<()> {
        let data = MyStruct {
            field1: "x".to_owned(),
            field2: -7,
        };
        let buffer = rkyv::to_bytes::<Error>(&data)?;
        let back = rkyv::from_bytes::<MyStruct, Error>(&buffer)?;
        assert_eq!(back, data);

        assert!(rkyv::from_bytes::<MyStruct, Error>(&[1, 2, 3]).is_err());
        Ok(())
    }
}
