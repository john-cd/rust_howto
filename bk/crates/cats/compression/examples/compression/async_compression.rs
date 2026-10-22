//! Compressing and decompressing a file asynchronously with
//! `async_compression`.
//!
//! TODO Reviewed ??-??-2026

#![allow(dead_code)]

// ANCHOR: example
use std::io::Result as IoResult;

use async_compression::tokio::bufread::GzipDecoder;
use async_compression::tokio::write::GzipEncoder;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;

/// Creates a file, compresses it to gzip, decompresses it and prints the
/// progress.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let input_filename = "temp/uncompressed_async.txt";
    let compressed_filename = "temp/compressed_async.gz";
    let decompressed_filename = "temp/decompressed_async.txt";

    // 1. Asynchronously create an uncompressed file.
    let mut input_file = File::create(input_filename).await?;
    input_file.write_all(b"This is some sample data.\n").await?;
    input_file.write_all(b"It has multiple lines.\n").await?;
    drop(input_file);

    println!("Created '{input_filename}'");

    // 2. Asynchronously compress the file.
    println!("Compressing '{input_filename}' to '{compressed_filename}'...");
    compress_file_async(input_filename, compressed_filename).await?;
    println!("Compression complete.");

    // 3. Asynchronously decompress the file.
    println!(
        "Decompressing '{compressed_filename}' to '{decompressed_filename}'..."
    );
    decompress_file_async(compressed_filename, decompressed_filename).await?;
    println!("Decompression complete.");

    // 4. Verify the decompressed content.
    let original_content = tokio::fs::read_to_string(input_filename).await?;
    let decompressed_content =
        tokio::fs::read_to_string(decompressed_filename).await?;

    if original_content == decompressed_content {
        println!("Decompressed content matches the original.");
    } else {
        eprintln!("Error: Decompressed content does NOT match the original!");
    }

    Ok(())
}

/// Compresses `source_path` into the gzip file `dest_path`.
async fn compress_file_async(
    source_path: &str,
    dest_path: &str,
) -> IoResult<()> {
    let mut source_file = File::open(source_path).await?;
    let dest_file = File::create(dest_path).await?;

    let mut encoder = GzipEncoder::new(dest_file);
    tokio::io::copy(&mut source_file, &mut encoder).await?;
    encoder.shutdown().await?;

    Ok(())
}

/// Decompresses the gzip file `source_path` into `dest_path`.
async fn decompress_file_async(
    source_path: &str,
    dest_path: &str,
) -> IoResult<()> {
    let source_file = File::open(source_path).await?;
    let mut dest_file = File::create(dest_path).await?;

    let reader = BufReader::new(source_file);
    let mut decoder = GzipDecoder::new(reader);
    tokio::io::copy(&mut decoder, &mut dest_file).await?;

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

    /// Checks that the example succeeds.
    #[test]
    fn test_main() -> anyhow::Result<()> {
        use std::fs;
        if !fs::exists("temp")? {
            fs::create_dir("temp")?;
        }
        main()?;
        Ok(())
    }
}
