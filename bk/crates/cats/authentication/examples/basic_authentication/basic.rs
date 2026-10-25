#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to use basic authentication with `reqwest`.

use anyhow::Context as _;
use reqwest::Url;
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

/// Sends a Basic-authenticated request using settings from the environment.
///
/// # Errors
/// Returns an error if a setting is missing, the URL is invalid or insecure, or
/// the request fails.
fn main() -> anyhow::Result<()> {
    let endpoint = std::env::var("BASIC_AUTH_URL")
        .context("set BASIC_AUTH_URL to an HTTPS endpoint protected by Basic authentication")?;
    let endpoint =
        Url::parse(&endpoint).context("BASIC_AUTH_URL must be a valid URL")?;
    anyhow::ensure!(
        endpoint.scheme() == "https",
        "BASIC_AUTH_URL must use HTTPS to protect credentials in transit"
    );
    let user_name = std::env::var("BASIC_AUTH_USERNAME")
        .context("set BASIC_AUTH_USERNAME to the account name")?;
    let password = std::env::var("BASIC_AUTH_PASSWORD").context(
        "set BASIC_AUTH_PASSWORD using your secret-management mechanism",
    )?;

    let client = Client::new();
    let response =
        basic_auth_request(&client, endpoint.as_str(), &user_name, &password)
            .send()?
            .error_for_status()?;
    println!("Authentication succeeded (HTTP {})", response.status());
    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
///
/// # Errors
/// Returns an error if a setting is missing, the URL is invalid or insecure, or
/// the request fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
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
