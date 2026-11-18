//! Runs the HTTP types example.
//!
//! TODO Reviewed ??-??-2026.

/// Builds and collects an in-memory HTTP body using the Body adapters.
mod body;
/// Constructs HTTP requests, responses, and headers with the http crate.
mod http;

/// Runs the HTTP types and interfaces example.
///
/// # Errors
///
/// Returns an error if header parsing or message construction fails.
fn main() -> anyhow::Result<()> {
    http::run()?;
    body::run()
}
