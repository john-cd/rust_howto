#![allow(dead_code)]
// ANCHOR: example
//! This example shows how to POST a file to a server.

use anyhow::Result;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<()> {
    let contents = fs::read_to_string("temp/message").await?;

    let httpbin_api = "https://httpbin.org/post";
    let client = reqwest::Client::new();
    let res = client.post(httpbin_api).body(contents).send().await?;
    let response_text = res.text().await?;
    println!("Response from httpbin: {response_text}");
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[tokio::test]
async fn test() -> anyhow::Result<()> {
    use tokio::io::AsyncWriteExt;
    if !tokio::fs::try_exists("temp").await? {
        tokio::fs::create_dir("temp").await?;
    }
    // Create a file to be posted.
    let mut f = File::create("temp/message").await?;
    f.write_all(b"Hello").await?;

    // In a CI environment, we might want to skip tests that require network
    // access.
    // TODO main().await?;
    Ok(())
}
