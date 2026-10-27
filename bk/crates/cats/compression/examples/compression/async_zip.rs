#![allow(dead_code)]
//! Writing and reading a ZIP archive asynchronously with `async_zip`.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example

use anyhow::Context as _;
use anyhow::Result;
use async_zip::Compression;
use async_zip::ZipEntryBuilder;
use async_zip::base::read1::seek::ZipArchiveReader;
use async_zip::base::write::ZipFileWriter;
use futures_lite::io::AsyncReadExt as _;
use futures_lite::io::Cursor;
use tokio::fs::File;
use tokio::io::AsyncReadExt as _;
use tokio_util::compat::TokioAsyncWriteCompatExt as _;

/// Writes a ZIP archive with one entry, reads it back, and checks its content.
///
/// # Errors
///
/// Returns an error if temporary storage, async file I/O, or ZIP processing
/// fails, or if the restored entry differs from the original.
#[tokio::main]
async fn main() -> Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let zip_path = temp_dir.path().join("archive.zip");

    // Write to zip
    let file = File::create(&zip_path).await?;
    let mut writer = ZipFileWriter::new(file.compat_write());

    let content = b"Hello, async world!";
    let entry =
        ZipEntryBuilder::new("hello.txt".into(), Compression::Stored).build();

    writer.write_entry_whole(entry, content).await?;
    writer.close().await?;

    // Read from zip
    let mut file = File::open(&zip_path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;

    let mut reader = ZipArchiveReader::open(Cursor::new(buffer)).await?;

    anyhow::ensure!(reader.cdrs().len() == 1, "expected one ZIP entry");
    for i in 0..reader.cdrs().len() {
        // File names in an archive are untrusted; here we only compare them.
        let filename = reader.cdrs()[i]
            .insecure_file_name
            .as_str()
            .context("ZIP filename is not UTF-8")?
            .to_owned();
        anyhow::ensure!(filename == "hello.txt", "unexpected ZIP entry name");
        println!("Entry {i}: {filename}");
        let mut entry_reader = reader.file(i).await?;
        let mut entry_content = String::new();
        entry_reader.read_to_string(&mut entry_content).await?;
        anyhow::ensure!(
            entry_content == "Hello, async world!",
            "unexpected ZIP entry content"
        );
    }

    Ok(())
}

// ANCHOR_END: example

/// Runs the asynchronous ZIP archive example.
///
/// # Errors
///
/// Returns an error if archive creation or reading fails.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the asynchronous archive round trip succeeds.
    ///
    /// # Errors
    ///
    /// Returns an error if the example fails.
    /// Runs the async zip example through its entry point.
    #[test]
    fn test_async_zip_example_entry_point() -> Result<()> {
        main()
    }
}
