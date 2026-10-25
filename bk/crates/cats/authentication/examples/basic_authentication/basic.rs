#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example builds a Basic-authenticated request without sending it.

use reqwest::blocking::Client;
use reqwest::blocking::RequestBuilder;

/// Creates a GET request that uses HTTP Basic authentication.
fn basic_auth_request(
    client: &Client,
    endpoint: &str,
    user_name: &str,
    password: &str,
) -> RequestBuilder {
    client.get(endpoint).basic_auth(user_name, Some(password))
}

/// Builds a request locally so the example can run without a network service.
///
/// The credentials are placeholders. Real credentials must be kept secret and
/// sent only to an HTTPS endpoint.
///
/// # Errors
/// Returns an error if the request cannot be constructed.
fn main() -> anyhow::Result<()> {
    let client = Client::new();
    let request = basic_auth_request(
        &client,
        "https://example.test/protected",
        "demo-user",
        "demo-password",
    )
    .build()?;
    println!(
        "Built a Basic-authenticated request for {} (not sent).",
        request.url()
    );
    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
///
/// # Errors
/// Returns an error if the request cannot be constructed.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use anyhow::Context as _;
    use reqwest::header::AUTHORIZATION;

    use super::*;

    /// Builds a request with an authorization header without sending it.
    #[test]
    fn basic_auth_header_is_added_without_network_access() -> anyhow::Result<()>
    {
        let request = basic_auth_request(
            &Client::new(),
            "https://example.test/protected",
            "alice",
            "secret",
        )
        .build()?;
        let authorization = request
            .headers()
            .get(AUTHORIZATION)
            .context("request should contain an Authorization header")?;

        assert_eq!(authorization, "Basic YWxpY2U6c2VjcmV0");
        Ok(())
    }
}
