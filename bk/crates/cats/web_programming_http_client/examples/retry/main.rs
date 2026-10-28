//! Retry with exponential backoff.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Retry a fallible async call with exponential backoff and jitter.
//!
//! [`backon`] is the recommended crate: `backoff` is unmaintained
//! (RUSTSEC-2025-0012). Only retry idempotent requests.

use std::time::Duration;

use anyhow::Result;
use anyhow::bail;
use backon::ExponentialBuilder;
use backon::Retryable;

/// Fetches `url`; fails on `429 Too Many Requests` and on server errors.
async fn fetch(url: &str) -> Result<String> {
    let response = reqwest::get(url).await?;
    let status = response.status();
    if status.is_server_error() || status.as_u16() == 429 {
        bail!("transient failure: {status}");
    }
    Ok(response.error_for_status()?.text().await?)
}

/// Retries `fetch` up to four times, doubling the delay each time.
async fn fetch_with_retry(url: &str) -> Result<String> {
    let backoff = ExponentialBuilder::default()
        .with_min_delay(Duration::from_millis(100))
        .with_max_delay(Duration::from_secs(5))
        .with_max_times(4)
        .with_jitter();

    (|| fetch(url))
        .retry(backoff)
        .sleep(tokio::time::sleep)
        .notify(|err, delay| println!("Retrying in {delay:?}: {err}"))
        .await
}

#[tokio::main]
async fn main() -> Result<()> {
    let body = fetch_with_retry("https://example.com").await?;
    println!("Received {} bytes", body.len());
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn retries_until_success() -> Result<()> {
        let mut server = mockito::Server::new_async().await;
        let failing = server
            .mock("GET", "/")
            .with_status(429)
            .with_header("retry-after", "0")
            .expect(2)
            .create_async()
            .await;
        let ok = server
            .mock("GET", "/")
            .with_status(200)
            .with_body("hello")
            .expect(1)
            .create_async()
            .await;

        let body = fetch_with_retry(&server.url()).await?;
        assert_eq!(body, "hello");
        failing.assert_async().await;
        ok.assert_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn gives_up_after_max_attempts() -> Result<()> {
        let mut server = mockito::Server::new_async().await;
        // One attempt plus four retries.
        let mock = server
            .mock("GET", "/")
            .with_status(503)
            .expect(5)
            .create_async()
            .await;
        assert!(fetch_with_retry(&server.url()).await.is_err());
        mock.assert_async().await;
        Ok(())
    }
}
