#![allow(dead_code)]

// ANCHOR: example
//! Serialize and deserialize a struct with MessagePack via `rmp-serde`.
//!
//! TODO Reviewed ??-??-2026

use serde::Deserialize;
use serde::Serialize;

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

    let encoded = rmp_serde::to_vec(&data)?;
    let decoded: MyData = rmp_serde::from_slice(&encoded)?;
    assert_eq!(data, decoded);

    println!("Encoded {} bytes", encoded.len());
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
    fn test_main() -> anyhow::Result<()> {
        main()
    }
    // [finish](https://github.com/john-cd/rust_howto/issues/1046)
}
