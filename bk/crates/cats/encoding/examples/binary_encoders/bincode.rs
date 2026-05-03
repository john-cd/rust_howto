#![allow(dead_code)]
// ANCHOR: example
//! Serialize and deserialize a struct with `bincode` 2.x.

use bincode::config::standard;
use bincode::serde::decode_from_slice;
use bincode::serde::encode_to_vec;
use serde::Deserialize;
use serde::Serialize;

/// Represents a struct that can be serialized and deserialized
/// using bincode.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct MyData {
    name: String,
    age: u32,
    is_active: bool,
}

fn main() -> anyhow::Result<()> {
    let data = MyData {
        name: "Alice".to_string(),
        age: 30,
        is_active: true,
    };

    let encoded = encode_to_vec(&data, standard())?;
    let (decoded, consumed): (MyData, usize) =
        decode_from_slice(&encoded, standard())?;

    println!(
        "Encoded {} bytes; decoder consumed {consumed} bytes",
        encoded.len()
    );
    println!("Decoded value: {decoded:?}");
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
    fn test() -> anyhow::Result<()> {
        main()
    }
    // [finish](https://github.com/john-cd/rust_howto/issues/1040)
}
