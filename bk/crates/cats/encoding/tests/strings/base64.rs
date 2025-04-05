// ANCHOR: example
//! This example demonstrates how to encode and decode a string using `base64`.

use std::str;

use anyhow::Result;
use base64::prelude::*;

fn main() -> Result<()> {
    let hello = b"hello rustaceans";
    let encoded: String = BASE64_STANDARD.encode(hello);
    let decoded: Vec<u8> = BASE64_STANDARD.decode(&encoded)?;

    println!("origin: {}", str::from_utf8(hello)?);
    println!("base64 encoded: {}", encoded);
    println!("back to origin: {}", str::from_utf8(&decoded)?);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
