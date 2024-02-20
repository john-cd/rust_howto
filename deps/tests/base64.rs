// use std::str;

// use anyhow::Result;
// use base64::decode;
// use base64::encode;

// #[test]
// fn test() -> Result<()> {
//     let hello = b"hello rustaceans";
//     let encoded = encode(hello);
//     let decoded = decode(&encoded)?;

//     println!("origin: {}", str::from_utf8(hello)?);
//     println!("base64 encoded: {}", encoded);
//     println!("back to origin: {}", str::from_utf8(&decoded)?);

//     Ok(())
// }
