#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Example of using the `flate2` crate.
//!
//! `flate2` is a stream compression library that supports DEFLATE, zlib,
//! and gzip formats. This example compresses bytes into a Zlib buffer,
//! then decompresses them back to text.

use std::io::Read as _;
use std::io::Write as _;

use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;

/// Compresses and decompresses a byte string with zlib and checks the round
/// trip.
fn main() -> anyhow::Result<()> {
    let input = b"Hello from flate2!";

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());
    encoder.write_all(input)?;
    let compressed = encoder.finish()?;

    let mut decoder = ZlibDecoder::new(compressed.as_slice());
    let mut decoded = String::new();
    decoder.read_to_string(&mut decoded)?;
    assert_eq!(decoded.as_bytes(), input);

    println!(
        "Compressed {} bytes into {} bytes",
        input.len(),
        compressed.len()
    );
    println!("Decoded text: {decoded}");
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }
    // [read / write a file](https://github.com/john-cd/rust_howto/issues/1009)
}
