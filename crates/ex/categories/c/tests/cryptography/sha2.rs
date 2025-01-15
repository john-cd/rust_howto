// // ANCHOR: example
// // ANCHOR_END: example

// use std::fs;
// use std::io;
// use base64ct::Base64;
// use base64ct::Encoding;
// use sha2::Digest;
// use sha2::Sha256;
// use sha2::Sha512;
// use sha2::{Digest, Sha256};

// fn main() {
//     // Create a Sha256 object
//     let mut hasher = Sha256::new();

//     // Write input data
//     let data = b"hello world";
//     hasher.update(data);

//     // `update` can be called repeatedly and is generic over `AsRef<[u8]>`
//     hasher.update("String data");

//     // Read hash digest and consume hasher
//     let result = hasher.finalize();

//     // Print the hash as a hexadecimal string
//     println!("SHA-256 hash of '{:?}': {:x}", data, result);

//     // same for Sha512
//     // Using `chain_update` instead of `update`:
//     let hash = Sha512::new()
//         .chain_update(b"Hello world!")
//         .chain_update("String data")
//         .finalize();

//     let base64_hash = Base64::encode_string(&hash);
//     println!("Base64-encoded hash: {}", base64_hash);

//     let hex_hash = base16ct::lower::encode_string(&hash);
//     println!("Hex-encoded hash: {}", hex_hash);
// }
// fn hash_message() -> Result<(), Box<dyn std::error::Error>> {

//     // If a complete message is available, then you can use the convenience
//     // Digest::digest method:
//     let hash = Sha256::digest(b"my message");
// }

//     // Hashing Readable Objects
// fn hash_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let mut file = fs::File::open(&path)?;
//     let mut hasher = Sha256::new();
//     let n = io::copy(&mut file, &mut hasher)?;
//     let hash = hasher.finalize();
// }

// #[test]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/703)
