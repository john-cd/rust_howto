#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Serialize and deserialize a struct with MessagePack via `rmp-serde`.

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

    let encoded = rmp_serde::to_vec(&data)?;
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

    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }
    // [finish](https://github.com/john-cd/rust_howto/issues/1046)
}
