#![allow(dead_code)]
// ANCHOR: example
// Constant-Time Base64 encoding
use base64ct::Base64;
use base64ct::Encoding;
// Convenience wrapper trait covering functionality of
// cryptographic hash functions with fixed output size.
use sha2::Digest;
// SHA-256 hasher
use sha2::Sha256;
// SHA-512 hasher.
use sha2::Sha512;

fn main() -> anyhow::Result<()> {
    // If a complete message is available, then you can use the convenience
    // `Digest::digest` method:
    let hash1 = Sha256::digest(b"my message");
    // Print the hash as a hexadecimal string
    println!("SHA-256 hash #1: {hash1:x}");

    // Otherwise, create a Sha256 hasher
    let mut hasher = Sha256::new();

    // Add input data
    let data = b"hello world";
    hasher.update(data);

    // `update` can be called repeatedly and is generic over `AsRef<[u8]>`
    hasher.update("String data");

    // Read hash digest and consume hasher
    let hash2 = hasher.finalize();
    println!("SHA-256 hash #2: {hash2:x}");

    // Same exercise, but using `Sha512` and `chain_update`:
    let hash3 = Sha512::new()
        .chain_update(b"Hello world!")
        // `chain_update` can be called repeatedly and is generic over `AsRef<[u8]>`
        .chain_update("String data")
        .finalize();

    let base64_hash = Base64::encode_string(&hash3);
    println!("Base64-encoded hash #3: {base64_hash}");

    // Hash the contents of a file:
    // First, we will create a file inside of `env::temp_dir()`.
    let mut file = tempfile::tempfile()?;
    use std::io::Write;
    writeln!(file, "Some data")?;
    // or: let mut file = fs::File::open(&some_path)?;

    // Copies the entire contents of a reader into a writer,
    // in this case the hasher
    let mut hasher = Sha256::new();

    std::io::copy(&mut file, &mut hasher)?;
    let hash4 = hasher.finalize();
    // Constant-time conversion to hexadecimal
    let hex_hash = base16ct::lower::encode_string(&hash4);
    println!("Hex-encoded hash #4: {hex_hash}");

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
