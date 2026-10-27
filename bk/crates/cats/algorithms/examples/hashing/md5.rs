#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! MD5 hashing with the RustCrypto `md-5` crate.
//!
//! WARNING: MD5 should be considered cryptographically broken and unsuitable
//! for further use. Collision attacks against MD5 are both practical and
//! trivial.

use md5::Digest as _;
use md5::Md5;

/// Prints the MD5 digest of a string.
fn main() {
    // Input data:
    let data = "hello world";

    // Compute MD5 hash:
    let digest = Md5::digest(data.as_bytes());

    // Print the hash as a hexadecimal string:
    println!(
        "MD5 hash of '{data}': {}",
        base16ct::lower::encode_string(&digest)
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the md5 example through its entry point.
    #[test]
    fn test_md5_example_entry_point() {
        main();
    }
}
