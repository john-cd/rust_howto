use std::str;

use anyhow::Result;
// use base64::{encode, decode};

fn main() -> Result<()> {
    let hello = b"hello rustaceans";
    // Use Engine::encode
    // let encoded = encode(hello);
    // let decoded = decode(&encoded)?;

    println!("origin: {}", str::from_utf8(hello)?);
    // println!("base64 encoded: {}", encoded);
    // println!("back to origin: {}", str::from_utf8(&decoded)?);

    Ok(())
}
