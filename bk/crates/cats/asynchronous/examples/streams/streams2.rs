#![allow(dead_code)]
#![allow(clippy::single_match)]
// ANCHOR: example
//! This example demonstrates downloading multiple files concurrently using
//! streams.
//!
//! It uses `reqwest` to download files from URLs and `tokio` for asynchronous
//! operations. The `futures` crate provides the `StreamExt` trait for working
//! with streams.

use std::fs;

use futures::StreamExt;
use tokio::fs::File;
use tokio::io;

type Result = std::result::Result<(), anyhow::Error>;

async fn download_file(url: &str, filename: &str) -> Result {
    let response = reqwest::get(url).await?;
    let content = response.bytes().await?;
    let mut file = File::create(filename).await?;
    io::copy(&mut content.as_ref(), &mut file).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result {
    let urls = ["https://www.gutenberg.org/cache/epub/43/pg43.txt"]; // Add more here...
    let filenames = ["temp/file1.txt"]; // Add more here...
    if !fs::exists("temp")? {
        fs::create_dir("temp")?;
    }

    // Create a stream of futures
    let futures = urls
        .iter()
        .zip(filenames.iter())
        .map(|(url, filename)| download_file(url, filename));

    let fut = futures::stream::iter(futures).for_each_concurrent(
        4,
        |fut| async move {
            match fut.await {
                Err(e) => {
                    println!("Error: {e}");
                    match e.source() {
                        // Print the source of the error if available
                        Some(source) => {
                            println!("  Caused by: {source}");
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        },
    );

    fut.await;

    println!("Downloaded files successfully!");
    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_network() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [finish; asynchronous/streams.md: add more. streams2.rs is noplayground because it requires a network. rewrite](https://github.com/john-cd/rust_howto/issues/645)
