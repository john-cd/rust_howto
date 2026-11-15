// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Honour `429 Too Many Requests` and `Retry-After`.
//!
//!
//! A server that is overloaded or rate limiting you answers `429 Too Many
//! Requests`, often with a `Retry-After` header that says how long to wait.
//! Honouring it, instead of retrying at once, avoids being blocked and gives
//!
//! the server time to recover.
//!
//! Retry after the delay the server asks for, when it asks for one.
//! `Retry-After` is either a number of seconds or an HTTP date; this example
//! parses the seconds form. [`backon`] computes the next delay, and
//! `adjust` replaces it with the server's value.

use std::fmt;
use std::time::Duration;

use anyhow::Result;
use backon::ExponentialBuilder;
use backon::Retryable;
use reqwest::StatusCode;
use reqwest::header::RETRY_AFTER;

/// A transient failure, with the delay requested by the server, if any.
#[derive(Debug)]
struct Throttled {
    /// Current lifecycle value used to decide which throttled operations are valid.
    status: StatusCode,
    /// Optional retry after used by throttled; `None` means the value is not configured or
    /// unavailable.
    retry_after: Option<Duration>,
}

impl fmt::Display for Throttled {
    /// Formats the value for display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "transient failure: {}", self.status)
    }
}

impl std::error::Error for Throttled {}

/// Parses the delay-seconds form of a `Retry-After` header value.
fn parse_retry_after(value: &str) -> Option<Duration> {
    value.trim().parse().ok().map(Duration::from_secs)
}
/// Fetches `url`; fails on `429` and `503`, keeping any `Retry-After`.
async fn fetch(url: &str) -> Result<String> {
    let response = reqwest::get(url).await?;
    let status = response.status();
    if status == StatusCode::TOO_MANY_REQUESTS || status == StatusCode::SERVICE_UNAVAILABLE {
        let retry_after = response
            .headers()
            .get(RETRY_AFTER)
            .and_then(|v| v.to_str().ok())
            .and_then(parse_retry_after);
        return Err(Throttled {
            status,
            retry_after,
        }
        .into());
    }
    Ok(response.error_for_status()?.text().await?)
}

/// Retries `fetch`, preferring the server's `Retry-After` over the backoff.
async fn fetch_with_retry_after(url: &str) -> Result<String> {
    let backoff = ExponentialBuilder::default()
        .with_min_delay(Duration::from_millis(100))
        .with_max_times(3);

    (|| fetch(url))
        .retry(backoff)
        .when(|err| err.downcast_ref::<Throttled>().is_some())
        .adjust(|err, computed| {
            let requested = err.downcast_ref::<Throttled>().and_then(|t| t.retry_after);
            // Cap the wait: never sleep longer than a minute.
            requested
                .map(|d| d.min(Duration::from_mins(1)))
                .or(computed)
        })
        .notify(|err, delay| println!("Retrying in {delay:?}: {err}"))
        .await
}

/// # Errors
/// An error is returned if a fallible operation fails.
async fn example_async() -> Result<()> {
    let body = fetch_with_retry_after("https://example.com").await?;
    println!("Received {} bytes", body.len());
    Ok(())
}

/// Runs the asynchronous HTTP example on a current-thread Tokio runtime.
fn example() -> Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(example_async())
}

/// Starts the example as a standalone program.
fn main() -> Result<()> {
    example()
}
// ANCHOR_END: example

/// Panics if an internal invariant is violated.
/// Tests retry delays and behavior for HTTP responses.
#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    /// Parses seconds only.
    #[test]
    fn parses_seconds_only() {
        assert_eq!(parse_retry_after(" 3 "), Some(Duration::from_secs(3)));
        assert_eq!(parse_retry_after("Wed, 21 Oct 2026 07:28:00 GMT"), None);
    }

    /// Verifies that the waits for the requested delay.
    #[tokio::test]
    async fn waits_for_the_requested_delay() -> Result<()> {
        let mut server = mockito::Server::new_async().await;
        let throttled = server
            .mock("GET", "/")
            .with_status(429)
            .with_header("retry-after", "1")
            .expect(1)
            .create_async()
            .await;
        let ok = server
            .mock("GET", "/")
            .with_status(200)
            .with_body("hello")
            .expect(1)
            .create_async()
            .await;

        let start = Instant::now();
        let body = fetch_with_retry_after(&server.url()).await?;
        // The computed backoff is about 100 ms; the header asks for 1 s.
        assert!(start.elapsed() >= Duration::from_secs(1));
        assert_eq!(body, "hello");
        throttled.assert_async().await;
        ok.assert_async().await;
        Ok(())
    }

    /// Verifies that the does not retry client errors.
    #[tokio::test]
    async fn does_not_retry_client_errors() -> Result<()> {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .with_status(404)
            .expect(1)
            .create_async()
            .await;
        assert!(fetch_with_retry_after(&server.url()).await.is_err());
        mock.assert_async().await;
        Ok(())
    }
}
