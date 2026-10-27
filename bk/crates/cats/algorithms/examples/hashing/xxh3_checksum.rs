#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! An integrity checksum with XXH3 (128-bit): append a tag to a buffer and
//! verify it on read.
//!
//! XXH3 is a fast, non-cryptographic hash. It detects accidental corruption
//! (truncated or bit-flipped files) but is **not** a MAC: an attacker can
//! forge a matching tag. For tamper resistance, use an HMAC or a signature.

use xxhash_rust::xxh3::xxh3_128;

/// The size of the appended tag, in bytes.
const TAG_LEN: usize = 16;

/// Returns the payload followed by its 16-byte XXH3 tag.
fn seal(payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(payload.len() + TAG_LEN);
    out.extend_from_slice(payload);
    out.extend_from_slice(&xxh3_128(payload).to_be_bytes());
    out
}

/// Verifies the tag and returns the payload, or `None` if it does not match.
fn open(sealed: &[u8]) -> Option<&[u8]> {
    let split = sealed.len().checked_sub(TAG_LEN)?;
    let (payload, tag) = sealed.split_at(split);
    let expected = xxh3_128(payload).to_be_bytes();
    (tag == expected).then_some(payload)
}

/// Seals a buffer, reads it back, and rejects a corrupted byte.
fn main() {
    let mut sealed = seal(b"cache entry");
    assert_eq!(open(&sealed), Some(&b"cache entry"[..]));

    // Flip one bit in the payload.
    sealed[0] ^= 0x01;
    assert_eq!(open(&sealed), None);

    // Too short to contain a tag.
    assert_eq!(open(b"short"), None);
}
// ANCHOR_END: example

#[allow(clippy::main_recursion)]
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that a corrupted checksum causes rejection.
    /// Runs the xxh3 checksum example through its entry point.
    #[test]
    fn test_xxh3_checksum_example_entry_point() {
        main();
    }
}
