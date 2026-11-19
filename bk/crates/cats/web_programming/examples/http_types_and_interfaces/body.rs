// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Builds and collects a small HTTP body.
//!
//! `http_body::Body` represents an asynchronous stream of frames. The
//! `http_body_util::Full` adapter creates a body from one byte buffer, and
//! `BodyExt::collect` buffers its frames into one result.

use bytes::Bytes;
use http_body::Body as _;
use http_body_util::BodyExt;
use http_body_util::Full;
use http_body_util::LengthLimitError;
use http_body_util::Limited;

/// Collects an in-memory body within its byte budget and rejects a larger one.
///
/// # Errors
///
/// Returns an error if collecting the body within the limit fails.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let body = Full::new(Bytes::from_static(b"hello from an HTTP body"));
    println!("Exact body size: {:?}", body.size_hint().exact());

    let bytes = Limited::new(body, b"hello from an HTTP body".len())
        .collect()
        .await
        .map_err(anyhow::Error::from_boxed)?
        .to_bytes();
    assert_eq!(bytes.as_ref(), b"hello from an HTTP body");
    println!("Collected body: {}", String::from_utf8_lossy(&bytes));

    let oversized = Full::new(Bytes::from_static(b"too long"));
    let result = Limited::new(oversized, 4).collect().await;
    assert!(matches!(result, Err(error) if error.is::<LengthLimitError>()));

    Ok(())
}
// ANCHOR_END: example

/// Runs the HTTP body example.
///
/// # Errors
///
/// Returns an error if the body example fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

/// Verifies the complete in-memory body example through its public entry flow.
#[cfg(test)]
mod tests {
    use super::*;

    /// Collects a body at the byte limit and rejects one over the limit.
    #[test]
    fn body_example_runs() -> anyhow::Result<()> {
        main()
    }
}
