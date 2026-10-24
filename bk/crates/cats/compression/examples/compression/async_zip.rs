#![allow(dead_code)]
//! Writing and reading a ZIP archive asynchronously with `async_zip`.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example

use anyhow::Context as _;
use anyhow::Result;
use async_zip::Compression;
use async_zip::ZipEntryBuilder;
use async_zip::base::read::mem::ZipFileReader;
use async_zip::base::write::ZipFileWriter;
use futures_lite::io::AsyncReadExt as _;
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

    let reader = ZipFileReader::new(buffer).await?;

    anyhow::ensure!(
        reader.file().entries().len() == 1,
        "expected one ZIP entry"
    );
    for (i, entry) in reader.file().entries().iter().enumerate() {
        let filename = entry
            .filename()
            .as_str()
            .context("ZIP filename is not UTF-8")?;
        anyhow::ensure!(filename == "hello.txt", "unexpected ZIP entry name");
        println!("Entry {i}: {filename}");
        let mut entry_reader = reader.reader_with_entry(i).await?;
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
    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}
