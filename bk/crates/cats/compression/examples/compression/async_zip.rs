// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Writing and reading a ZIP archive asynchronously with `async_zip`.

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

    // Create an archive and write one entry to it.
    let file = File::create(&zip_path).await?;
    let mut writer = ZipFileWriter::new(file.compat_write());

    let content = b"Hello, async world!";
    let entry = ZipEntryBuilder::new("hello.txt".into(), Compression::Stored).build();

    writer.write_entry_whole(entry, content).await?;
    writer.close().await?;

    // Read the archive back so the example can verify the round trip.
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
/// Writing and reading a ZIP archive asynchronously with `async_zip`.
///
/// # Errors
///
/// Returns an error if archive creation or reading fails.
/// Returns an error if a step of the example fails.
pub fn run() -> Result<()> {
    main()
}

/// Tests the asynchronous ZIP example behavior.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point without error.
    #[test]
    fn test_async_zip_example_entry_point() -> Result<()> {
        main()
    }

    /// Verifies that the archive with several entries lists them in order.
    #[tokio::test]
    async fn archive_with_several_entries_lists_them_in_order() -> Result<()> {
        let mut writer = ZipFileWriter::new(Vec::new().compat_write());
        for (name, data) in [("a.txt", "first"), ("dir/b.txt", "second")] {
            let entry = ZipEntryBuilder::new(name.into(), Compression::Stored).build();
            writer.write_entry_whole(entry, data.as_bytes()).await?;
        }
        let bytes = writer.close().await?.into_inner();

        let mut reader = ZipArchiveReader::open(Cursor::new(bytes)).await?;
        let names: Vec<String> = reader
            .cdrs()
            .iter()
            .map(|c| c.insecure_file_name.as_str().map(str::to_owned))
            .collect::<Option<_>>()
            .ok_or_else(|| anyhow::anyhow!("non-UTF-8 file name"))?;
        assert_eq!(names, ["a.txt", "dir/b.txt"]);

        let mut content = String::new();
        reader.file(1).await?.read_to_string(&mut content).await?;
        assert_eq!(content, "second");
        Ok(())
    }

    /// Reports an error when opening garbage occurs.
    #[tokio::test]
    async fn opening_garbage_is_an_error() {
        let result = ZipArchiveReader::open(Cursor::new(b"not a zip".to_vec())).await;
        assert!(result.is_err());
    }
}
