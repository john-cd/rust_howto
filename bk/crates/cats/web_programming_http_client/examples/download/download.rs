// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Downloads a file as binary response chunks into a temporary directory.

use std::path::Path;

use anyhow::Context as _;
use anyhow::Result;
use futures_util::TryStreamExt as _;
use tempfile::Builder;
use tokio::io::AsyncWriteExt as _;

/// Streams a successful HTTP response into a file and returns its byte count.
///
/// # Errors
///
/// Returns an error if the request, response status, file creation, streamed
/// body, byte count, or file write fails.
async fn download_to(url: &str, destination: &Path) -> Result<u64> {
    let response = reqwest::get(url).await?.error_for_status()?;
    let mut chunks = response.bytes_stream();
    let mut destination = tokio::fs::File::create(destination).await?;
    let mut downloaded = 0_u64;

    while let Some(chunk) = chunks.try_next().await? {
        destination.write_all(&chunk).await?;
        downloaded = downloaded
            .checked_add(u64::try_from(chunk.len())?)
            .context("downloaded byte count overflowed")?;
    }

    destination.flush().await?;
    Ok(downloaded)
}

/// Downloads a sample image into a temporary directory.
///
/// # Errors
///
/// Returns any temporary-directory, request, response, or file-write error.
#[tokio::main]
async fn main() -> Result<()> {
    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let destination = tmp_dir.path().join("rust-logo-512x512.png");
    let downloaded = download_to(target, &destination).await?;

    println!("downloaded {downloaded} bytes to {destination:?}");
    Ok(())
}

// ANCHOR_END: example

/// Runs the asynchronous binary download example.
///
/// # Errors
///
/// Returns any error from the asynchronous download.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use mockito::Server;

    use super::download_to;
    use super::main;

    /// Marks the live image download as an opt-in network test.
    #[test]
    #[ignore = "needs network"]
    fn require_network() -> Result<()> {
        main()?;
        Ok(())
    }

    /// Verifies streaming preserves arbitrary binary bytes from a local server.
    #[tokio::test]
    async fn downloads_binary_response_bytes() -> Result<()> {
        let body = b"\0\x01\x02\xffPNG";
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/file")
            .with_status(200)
            .with_body(body)
            .create_async()
            .await;
        let directory = tempfile::tempdir()?;
        let destination = directory.path().join("download.bin");

        let downloaded =
            download_to(&format!("{}/file", server.url()), &destination)
                .await?;

        mock.assert_async().await;
        assert_eq!(downloaded, 7);
        assert_eq!(tokio::fs::read(destination).await?, b"\0\x01\x02\xffPNG");
        Ok(())
    }
}
