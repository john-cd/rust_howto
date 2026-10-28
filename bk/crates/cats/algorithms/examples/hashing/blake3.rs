// TODO Reviewed ??-??-2026.
// TODO short explaination BLAKE3

// ANCHOR: example
//! Cryptographic hashing with `blake3`: one-shot, incremental, keyed, key
//! derivation, and extensible output.
//!
//! ### What is BLAKE3?
//! BLAKE3 is a modern cryptographic hash function that is significantly faster
//! than MD5, SHA-1, SHA-2, SHA-3, and BLAKE2, while providing 128 bits of
//! cryptographic security against all known attacks.
//!
//! ### Underlying Technology & Architecture
//! - **Tree Hashing**: Internally, BLAKE3 structures the input into a binary
//!   Merkle tree of 1024-byte chunks. This architecture allows computations to
//!   be split across multiple CPU cores or parallelized using SIMD vector
//!   instructions (such as AVX-512 or NEON).
//! - **Bao & BLAKE2s Heritage**: BLAKE3 is based on an optimized variation of
//!   the BLAKE2s hash function and the Bao tree hashing scheme.
//!
//! ### Purpose & Use Cases
//! - **Integrity Checking**: Computing hash digests for file verification,
//!   caching, and content-addressable storage systems.
//! - **Streaming & Incremental Hashing**: Processing large files or network
//!   streams in chunks without loading all data into memory at once.
//! - **Message Authentication (MAC)**: Authenticating messages with a shared
//!   secret key to detect unauthorized modifications.
//! - **Key Derivation Function (KDF)**: Generating domain-specific subkeys from
//!   master secrets using contextual salt strings.
//! - **Extensible Output Function (XOF)**: Generating variable-length
//!   pseudo-random outputs of arbitrary byte length.
//!
//! ### Password Hashing Warning
//! BLAKE3 is designed for maximum speed and efficiency. It is **not** suitable
//! for password hashing or password storage, because fast algorithms allow
//! attackers to run rapid brute-force dictionary attacks. Use a slow,
//! memory-hard algorithm such as `argon2id` for passwords.

use blake3::Hasher;

/// Demonstrates six primary usage patterns of the BLAKE3 cryptographic hash
/// function.
fn main() {
    // Example 1: One-shot hashing of a simple string.
    // `blake3::hash` computes the 256-bit (32-byte) hash in a single step for
    // data fully present in memory.
    let input_string = "Hello, world!";
    let hash = blake3::hash(input_string.as_bytes());
    println!("Hash of '{input_string}': {hash}");

    // Example 2: Incremental (streaming) hashing.
    // When dealing with large files or network streams, feed data in chunks
    // via `Hasher::update()` and finalize the result once all data is ingested.
    let mut hasher = Hasher::new();
    hasher.update(b"The quick brown ");
    hasher.update(b"fox jumps over ");
    hasher.update(b"the lazy dog.");
    let hash2 = hasher.finalize();
    println!("Incremental hash: {hash2}");

    // Example 3: Hashing a larger byte array.
    // BLAKE3 automatically applies tree hashing internally to parallelize the
    // computation over large byte slices.
    let large_data: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();
    let hash3 = blake3::hash(&large_data);
    println!("Hash of 1KB data: {hash3}");

    // Example 4: Keyed hashing as a Message Authentication Code (MAC).
    // Uses a 32-byte secret key shared between sender and recipient to verify
    // both data integrity and authenticity, preventing forgery.
    let key: &[u8; 32] = b"mysecretkey__________-----------";
    let mut hasher_keyed = blake3::Hasher::new_keyed(key);
    hasher_keyed.update(b"Message to be keyed hashed");
    let keyed_hash = hasher_keyed.finalize();
    println!("Keyed hash: {keyed_hash}");

    // Example 5: Deriving subkeys using an application context string.
    // Context-separated key derivation converts master cryptographic key
    // material into application-specific subkeys. The context string must be
    // hardcoded, globally unique, and specific to the application domain to
    // prevent cross-protocol attacks.
    let context = "My application context";
    let derived_key = blake3::derive_key(context, b"Input key material");
    println!("Derived Key: {derived_key:?}");

    // Example 6: Extensible Output Function (XOF).
    // An XOF produces an arbitrarily long output digest stream, useful for
    // expanding seed material, masking, or key expansion.
    let mut output = [0u8; 1000];
    let hasher = blake3::Hasher::new();
    let mut output_reader = hasher.finalize_xof();
    output_reader.fill(&mut output);
    println!("Output: {output:x?}");
}
// ANCHOR_END: example

/// Runs the BLAKE3 example by delegating to `main()`.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_example_entry_point() {
        main();
    }
}
