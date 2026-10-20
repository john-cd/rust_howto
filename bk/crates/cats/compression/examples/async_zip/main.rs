use anyhow::Result;
use async_zip::Compression;
use async_zip::ZipEntryBuilder;
use async_zip::base::read::mem::ZipFileReader;
use async_zip::base::write::ZipFileWriter;
use futures_lite::io::AsyncReadExt as FuturesAsyncReadExt;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio_util::compat::TokioAsyncWriteCompatExt;

#[tokio::main]
async fn main() -> Result<()> {
    println!("async_zip example");

    // Create a temporary zip file
    let temp_dir = std::env::temp_dir();
    let zip_path = temp_dir.join("test_async.zip");

    // Write to zip
    let file = File::create(&zip_path).await?;
    let mut writer = ZipFileWriter::new(file.compat_write());

    let content = b"Hello, async world!";
    let entry =
        ZipEntryBuilder::new("hello.txt".into(), Compression::Stored).build();

    writer.write_entry_whole(entry, content).await?;
    writer.close().await?;

    println!("Successfully wrote zip file to {:?}", zip_path);

    // Read from zip
    let mut file = File::open(&zip_path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;

    let reader = ZipFileReader::new(buffer).await?;

    for (i, entry) in reader.file().entries().iter().enumerate() {
        println!("Entry {}: {}", i, entry.filename().as_str().unwrap());
        let mut entry_reader = reader.reader_with_entry(i).await?;
        let mut entry_content = String::new();
        entry_reader.read_to_string(&mut entry_content).await?;
        println!("Content: {}", entry_content);
    }

    // Clean up
    tokio::fs::remove_file(zip_path).await?;

    Ok(())
}
