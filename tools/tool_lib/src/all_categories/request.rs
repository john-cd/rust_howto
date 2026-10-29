//! Downloading the categories file of `crates.io`.
//!
//! TODO Reviewed ??-??-2026.

use anyhow::Result;

// The master list of categories is a TOML file in the `crates.io` repo:
/// URL of the categories TOML file.
static CATEGORIES_URL: &str = "https://raw.githubusercontent.com/rust-lang/crates.io/refs/heads/main/src/boot/categories.toml";

/// Downloads the categories TOML file, or the mock URL in
/// `MOCK_CATEGORIES_URL`.
pub(super) fn get_categories_toml_string() -> Result<String> {
    // Allows tests to override the URL.
    let url_str =
        std::env::var("MOCK_CATEGORIES_URL").unwrap_or_else(|_| CATEGORIES_URL.to_owned());
    let url = reqwest::Url::parse(&url_str)?;

    let is_localhost = url.host_str() == Some("127.0.0.1") || url.host_str() == Some("localhost");
    let builder = reqwest::blocking::Client::builder().https_only(!is_localhost);

    #[cfg(test)]
    let builder = builder.no_proxy(); // Important so mockito requests don't get routed through ALL_PROXY when running locally

    let client = builder.build()?;
    let response = client.get(url).send()?;
    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Request failed with status: {}",
            response.status()
        ));
    }
    let body = response.text()?;
    Ok(body)
}

#[cfg(test)]
#[expect(unsafe_code, reason = "tests call the unsafe EnvGuard under ENV_MUTEX")]
mod tests {
    use anyhow::Result;

    use super::*;
    use crate::ENV_MUTEX;
    use crate::utils::EnvGuard;

    /// Verifies that the category TOML request returns the server response.
    #[test]
    fn test_get_categories_toml_string_success() -> Result<()> {
        let _lock = ENV_MUTEX
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut server = mockito::Server::new();
        let expected_toml = "[category1]\nname = \"Category One\"\ndescription = \"First category\"\n\n[category2]\nname = \"Category Two\"\n";

        // Mock the GET request to the specific path on the mock server
        let mock_endpoint = server
            .mock("GET", "/")
            .with_status(200)
            .with_header("content-type", "text/plain; charset=utf-8") // Match typical GitHub raw response.
            .with_body(expected_toml)
            .create();

        let mock_url = format!("{}/", server.url());
        // SAFETY: serialized by `ENV_MUTEX`; no other thread uses the environment.
        let _url_guard = unsafe { EnvGuard::set("MOCK_CATEGORIES_URL", &mock_url) };

        let result = get_categories_toml_string()?;

        mock_endpoint.assert();
        assert_eq!(result, expected_toml);

        Ok(())
    }

    /// Verifies that an unsuccessful category TOML request returns an error.
    #[test]
    fn test_get_categories_toml_string_error() -> Result<()> {
        let _lock = ENV_MUTEX
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut server = mockito::Server::new();

        // Mock the GET request to return a 404 error.
        let mock_endpoint = server
            .mock("GET", "/")
            .with_status(404)
            .with_body("Not Found")
            .create();

        let mock_url = format!("{}/", server.url());
        // SAFETY: serialized by `ENV_MUTEX`; no other thread uses the environment.
        let _url_guard = unsafe { EnvGuard::set("MOCK_CATEGORIES_URL", &mock_url) };

        let error =
            get_categories_toml_string().expect_err("a 404 response should fail the request");
        mock_endpoint.assert();

        let err_string = error.to_string();
        assert!(
            err_string.contains("404") || err_string.contains("Not Found"),
            "Error message did not indicate a 404 error: {err_string}"
        );

        Ok(())
    }
}
