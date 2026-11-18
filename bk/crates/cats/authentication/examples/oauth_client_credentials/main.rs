//! Runs the OAuth 2.0 client-credentials example.
//!
//!
//! TODO Reviewed ??-??-2026.

/// Exercises a service token exchange and protected API call against local mocks.
mod client_credentials;

/// Runs the OAuth 2.0 client-credentials example.
fn main() -> anyhow::Result<()> {
    client_credentials::run()
}
