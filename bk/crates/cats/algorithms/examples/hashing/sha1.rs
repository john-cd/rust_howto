#![allow(dead_code)]
// ANCHOR: example
use hex_literal::hex;
use sha1::Digest;
use sha1::Sha1;

// BEWARE: SHA-1 is considered cryptographically broken and should NOT be used
// for new security-critical applications. It is primarily used for legacy
// compatibility or non-security-sensitive purposes.

fn main() {
    // Create a SHA-1 hasher:
    let mut hasher = Sha1::new();

    // Process input message:
    hasher.update(b"hello world");

    // Compute the hash digest:
    let result = hasher.finalize();

    // Assert the expected hash value:
    assert_eq!(result[..], hex!("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"));

    let hex_result = base16ct::lower::encode_string(&result);
    println!("SHA-1 hash of 'hello world': {hex_result}");
}
// Example adapted from <https://docs.rs/sha1/0.10.6/sha1/index.html>.
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        main();
    }
}
