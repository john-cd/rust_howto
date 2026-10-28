#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Cryptographic hashing with the `sha2` crate: SHA-256 and SHA-512 digests
//! formatted as hexadecimal or Base64.
//!
//! ### What is SHA-2?
//! SHA-2 (Secure Hash Algorithm 2) is a family of cryptographic hash functions
//! standardized by NIST (FIPS PUB 180-4). The most widely used variants are
//! SHA-256 (256-bit output digest) and SHA-512 (512-bit output digest).
//!
//! ### Underlying Architecture
//! SHA-2 uses the **Merkle–Damgård construction** with a Davis–Meyer
//! compression function. It converts arbitrary-length input data into
//! fixed-size byte arrays.
//!
//! ### Core Security Guarantees
//! - **Pre-image Resistance**: Given a hash digest $H$, it is computationally
//!   infeasible to find any input message $M$ such that $\text{hash}(M) = H$.
//! - **Collision Resistance**: It is computationally infeasible to find two
//!   distinct inputs $M_1 \neq M_2$ such that $\text{hash}(M_1) =
//!   \text{hash}(M_2)$.
//!
//! ### Purpose & Use Cases
//! - Digital signatures, TLS/SSL certificate verification, and Git commit IDs.
//! - Cryptographic integrity checking for software releases and download
//!   packages.
//! - Proof-of-work algorithms (such as Bitcoin mining).

// Constant-Time Base64 encoding.
use base64ct::Base64;
use base64ct::Encoding as _;
// Convenience trait providing unified cryptographic digest methods.
use sha2::Digest as _;
// SHA-256 hasher struct.
use sha2::Sha256;
// SHA-512 hasher struct.
use sha2::Sha512;

/// Demonstrates computing SHA-256 and SHA-512 cryptographic digests for strings
/// and files, encoding results in hexadecimal or Base64.
fn main() -> anyhow::Result<()> {
    // Convenience single-shot hash computation for data fully present in
    // memory.
    let hash1 = Sha256::digest(b"my message");
    println!(
        "SHA-256 hash #1: {}",
        base16ct::lower::encode_string(&hash1)
    );

    // Incremental streaming hash computation using a `Sha256` hasher instance.
    let mut hasher = Sha256::new();

    // Feed input bytes incrementally using `update`.
    let data = b"hello world";
    hasher.update(data);
    hasher.update("String data");

    // Finalize digest state to consume hasher and extract result.
    let hash2 = hasher.finalize();
    println!(
        "SHA-256 hash #2: {}",
        base16ct::lower::encode_string(&hash2)
    );

    // Method chaining style with `Sha512` and `chain_update`.
    let hash3 = Sha512::new()
        .chain_update(b"Hello world!")
        .chain_update("String data")
        .finalize();

    // Encode SHA-512 digest as constant-time Base64 string.
    let base64_hash = Base64::encode_string(&hash3);
    println!("Base64-encoded hash #3: {base64_hash}");

    // File hashing example: compute SHA-256 digest over file contents.
    let mut file = tempfile::tempfile()?;

    use std::io::Read as _;
    use std::io::Seek as _;
    use std::io::Write as _;

    writeln!(file, "Some data")?;

    file.rewind()?;
    let mut file_contents = Vec::new();
    file.read_to_end(&mut file_contents)?;
    let hash4 = Sha256::digest(&file_contents);

    let hex_hash = base16ct::lower::encode_string(&hash4);
    println!("Hex-encoded hash #4: {hex_hash}");

    Ok(())
}

// ANCHOR_END: example

/// Runs the `sha2` example by calling `main()`.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
/// Unit tests for the `sha2` example.
mod tests {
    use super::*;

    /// Runs the SHA-2 hashing example through its `main()` entry point.
    #[test]
    fn test_sha2_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
