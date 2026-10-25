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

// ANCHOR: gzip-file
/// Compresses a source file to gzip and restores it to another temporary file.
///
/// # Errors
///
/// Returns an error if a temporary file cannot be created or any file
/// operation or compression step fails.
fn gzip_file_round_trip() -> anyhow::Result<()> {
    use std::io::Write as _;

    use flate2::Compression;
    use flate2::read::GzDecoder;
    use flate2::write::GzEncoder;
    use tempfile::NamedTempFile;

    let original = b"File contents compressed and restored with gzip.";

    let mut source_file = NamedTempFile::new()?;
    source_file.write_all(original)?;

    let compressed_file = NamedTempFile::new()?;
    let mut source = source_file.reopen()?;
    let mut encoder =
        GzEncoder::new(compressed_file.reopen()?, Compression::fast());
    std::io::copy(&mut source, &mut encoder)?;
    encoder.finish()?;

    let mut restored_file = NamedTempFile::new()?;
    let mut decoder = GzDecoder::new(compressed_file.reopen()?);
    std::io::copy(&mut decoder, restored_file.as_file_mut())?;

    let restored = std::fs::read(restored_file.path())?;
    assert_eq!(restored, original);
    println!(
        "Gzip-compressed {} bytes into {} bytes",
        original.len(),
        compressed_file.as_file().metadata()?.len()
    );
    println!(
        "Restored file contents: {}",
        String::from_utf8_lossy(&restored)
    );
    Ok(())
}
// ANCHOR_END: gzip-file

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main()?;
    gzip_file_round_trip()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks the in-memory and temporary-file compression round trips.
    #[test]
    fn test_main() -> anyhow::Result<()> {
        run()
    }
}
