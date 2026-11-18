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

/// Collects a small in-memory body and checks its contents.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let body = Full::new(Bytes::from_static(b"hello from an HTTP body"));
    println!("Exact body size: {:?}", body.size_hint().exact());

    let bytes = body
        .collect()
        .await
        .expect("a `Full` body is infallible")
        .to_bytes();
    assert_eq!(bytes.as_ref(), b"hello from an HTTP body");
    println!("Collected body: {}", String::from_utf8_lossy(&bytes));

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

    /// Runs the small body example through its entry point.
    #[test]
    fn body_example_runs() -> anyhow::Result<()> {
        main()
    }
}
