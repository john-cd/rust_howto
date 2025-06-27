#![allow(dead_code)]
// ANCHOR: example
//! MD5 hashing example.
//!
//! WARNING: MD5 should be considered cryptographically broken and unsuitable
//! for further use. Collision attacks against MD5 are both practical and
//! trivial.
//!
//! The `md5` crate does not implement the `digest` traits, so it is not
//! interoperable with the RustCrypto ecosystem.
fn main() {
    // Input data
    let data = "hello world";

    // Compute MD5 hash
    let digest = md5::compute(data);

    // Print the hash as a hexadecimal string
    println!("MD5 hash of '{data}': {digest:x}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
