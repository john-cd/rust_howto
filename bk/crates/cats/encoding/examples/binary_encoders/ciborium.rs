#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! This example show how to (de)serialize CBOR data using the ciborium crate.
//!
//! CBOR stands for Concise Binary Object Representation, a binary data
//! serialization format. It is based on the JSON data model but is designed to
//! be more compact and efficient, making it ideal for applications like the
//! Internet of Things (IoT) and other scenarios where small message sizes and
//! fast processing are crucial. CBOR supports numbers, strings, arrays, maps,
//! and values like true, false, and null. Unlike JSON, it can handle binary
//! data directly, avoiding the need for encoding methods like 'Base64'.

use std::io::Cursor;

use ciborium::de::from_reader;
use ciborium::ser::into_writer;
// Import the `Serialize` and `Deserialize` traits from the `serde` crate.
use serde::Deserialize;
use serde::Serialize;

/// A struct that is serialized to and deserialized from CBOR.
#[derive(Serialize, Deserialize, Debug)]
struct MyData {
    /// The name.
    name: String,
    /// The age in years.
    age: u32,
    /// Whether the data is active.
    is_active: bool,
}

/// Demonstrates serializing and deserializing a struct to and
/// from CBOR.
fn main() -> anyhow::Result<()> {
    // Create a sample data object.
    let data = MyData {
        name: "Alice".to_owned(),
        age: 30,
        is_active: true,
    };

    // Serialize to CBOR.
    let mut buf = Vec::new();
    into_writer(&data, &mut buf)?;

    // Deserialize from CBOR.
    // (the `Cursor` just wraps the in-memory buffer).
    let mut cursor = Cursor::new(&*buf);
    let deserialized_data: MyData = from_reader(&mut cursor)?;

    // Print the deserialized data.
    println!("{deserialized_data:?}");

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

    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
