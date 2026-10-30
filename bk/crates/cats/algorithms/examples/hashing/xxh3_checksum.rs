// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Ultra-fast data integrity verification using XXH3 (128-bit) checksums.
//!
//! ### What is XXH3?
//! XXH3 is a state-of-the-art non-cryptographic hash algorithm from the xxHash
//! family designed to process data at near-memory-bandwidth speeds (tens of
//! gigabytes per second) by leveraging SIMD vector instructions (AVX2, SSE2,
//! NEON).
//!
//! ### Non-Cryptographic Checksum vs. Cryptographic MAC
//! - **Accidental Corruption Detection**: XXH3 reliably detects accidental bit
//!   flips, truncated files, network transmission errors, and disk corruption.
//! - **NOT Malice-Proof**: Because XXH3 is non-cryptographic, an intentional
//!   adversary can easily forge a valid matching XXH3 checksum for tampered
//!   payload data. For tamper resistance against malicious attackers, use HMAC
//!   (e.g., `HMAC-SHA256`) or digital signatures.
//!
//! ### Purpose & Use Cases
//! - In-memory data frame checksums in high-throughput network streaming.
//! - Database page integrity checking and disk block validation.
//! - Cache validation and fast deduplication checks.

use xxhash_rust::xxh3::xxh3_128;

/// The byte length of the appended 128-bit (16-byte) XXH3 checksum tag.
const TAG_LEN: usize = 16;

/// Seals a payload by appending its 16-byte XXH3 checksum tag to the buffer
/// end.
fn seal(payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(payload.len() + TAG_LEN);
    out.extend_from_slice(payload);
    out.extend_from_slice(&xxh3_128(payload).to_be_bytes());
    out
}

/// Opens a sealed buffer, verifies its appended XXH3 checksum tag, and returns
/// the payload slice if valid, or `None` if corrupted.
fn open(sealed: &[u8]) -> Option<&[u8]> {
    let split = sealed.len().checked_sub(TAG_LEN)?;
    let (payload, tag) = sealed.split_at(split);
    let expected = xxh3_128(payload).to_be_bytes();
    (tag == expected).then_some(payload)
}

/// Seals a data buffer, verifies it upon re-reading, and rejects corrupted
/// data.
fn main() {
    // Construct a sealed buffer containing the original message and tag.
    let mut sealed = seal(b"cache entry");
    assert_eq!(open(&sealed), Some(&b"cache entry"[..]));

    // Simulate accidental corruption by flipping a single bit in the payload.
    sealed[0] ^= 0x01;
    assert_eq!(open(&sealed), None);

    // Reject input buffers that are too short to contain a valid checksum tag.
    assert_eq!(open(b"short"), None);
}

// ANCHOR_END: example

/// Runs the XXH3 checksum example by calling `main()`.
pub fn run() {
    // Call through a binding: calling `main` directly trips `clippy::main_recursion`.
    let entry = main;
    entry();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xxh3_checksum_example_entry_point() {
        main();
    }
}
