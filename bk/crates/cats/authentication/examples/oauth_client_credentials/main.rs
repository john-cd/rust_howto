//! Runs the OAuth 2.0 client-credentials example.
//!
//! TODO Reviewed 10-27-2026.

mod client_credentials;

/// Runs the OAuth 2.0 client-credentials example.
fn main() -> anyhow::Result<()> {
    client_credentials::run()
}
