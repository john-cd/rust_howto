// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Compressing and decompressing a file asynchronously with
//! `async_compression`.

use std::io::Result as IoResult;
use std::path::Path;

use async_compression::tokio::bufread::GzipDecoder;
use async_compression::tokio::write::GzipEncoder;
use tempfile::tempdir;
use tokio::fs::File;
use tokio::io::AsyncWriteExt as _;
use tokio::io::BufReader;

/// Creates a file, compresses it to gzip, decompresses it and prints the
/// progress.
///
/// # Errors
///
/// Returns an error if temporary storage, file I/O, compression, decompression,
/// or the round-trip comparison fails.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let temp_dir = tempdir()?;
    let input_filename = temp_dir.path().join("uncompressed_async.txt");
    let compressed_filename = temp_dir.path().join("compressed_async.gz");
    let decompressed_filename = temp_dir.path().join("decompressed_async.txt");

    // 1. Asynchronously create an uncompressed file.
    let mut input_file = File::create(&input_filename).await?;
    input_file.write_all(b"This is some sample data.\n").await?;
    input_file.write_all(b"It has multiple lines.\n").await?;
    drop(input_file);

    println!("Created '{}'", input_filename.display());

    // 2. Asynchronously compress the file.
    println!(
        "Compressing '{}' to '{}'...",
        input_filename.display(),
        compressed_filename.display()
    );
    compress_file_async(&input_filename, &compressed_filename).await?;
    println!("Compression complete.");

    // 3. Asynchronously decompress the file.
    println!(
        "Decompressing '{}' to '{}'...",
        compressed_filename.display(),
        decompressed_filename.display()
    );
    decompress_file_async(&compressed_filename, &decompressed_filename).await?;
    println!("Decompression complete.");

    // 4. Verify the decompressed content.
    let original_content = tokio::fs::read_to_string(&input_filename).await?;
    let decompressed_content =
        tokio::fs::read_to_string(&decompressed_filename).await?;
    anyhow::ensure!(
        original_content == decompressed_content,
        "decompressed content differs from the original"
    );
    println!("Decompressed content matches the original.");

    Ok(())
}

/// Compresses `source_path` into the gzip file `dest_path`.
///
/// # Errors
///
/// Returns an I/O error if either file cannot be opened or copied.
async fn compress_file_async(
    source_path: &Path,
    dest_path: &Path,
) -> IoResult<()> {
    let mut source_file = File::open(source_path).await?;
    let dest_file = File::create(dest_path).await?;

    let mut encoder = GzipEncoder::new(dest_file);
    tokio::io::copy(&mut source_file, &mut encoder).await?;
    encoder.shutdown().await?;

    Ok(())
}

/// Decompresses the gzip file `source_path` into `dest_path`.
///
/// # Errors
///
/// Returns an I/O error if either file cannot be opened or copied.
async fn decompress_file_async(
    source_path: &Path,
    dest_path: &Path,
) -> IoResult<()> {
    let source_file = File::open(source_path).await?;
    let mut dest_file = File::create(dest_path).await?;

    let reader = BufReader::new(source_file);
    let mut decoder = GzipDecoder::new(reader);
    tokio::io::copy(&mut decoder, &mut dest_file).await?;

    Ok(())
}
// ANCHOR_END: example

/// # Errors
///
/// Returns an error if compression or decompression fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the example succeeds.
    ///
    /// # Errors
    ///
    /// Returns an error if the round trip fails.
    /// Runs the async compression example through its entry point.
    #[test]
    fn test_async_compression_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
