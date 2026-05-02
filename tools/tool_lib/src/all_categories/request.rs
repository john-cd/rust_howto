use anyhow::Result;

// The master list of categories is a TOML file in the `crates.io` repo:
static CATEGORIES_URL: &str = "https://raw.githubusercontent.com/rust-lang/crates.io/refs/heads/main/src/boot/categories.toml";

pub(super) fn get_categories_toml_string() -> Result<String> {
    // Allows tests to override the URL.
    let url_str =
        std::env::var("MOCK_CATEGORIES_URL").unwrap_or_else(|_| CATEGORIES_URL.to_string());
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

#[allow(dead_code)]
#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    // A mutex to ensure that tests setting MOCK_CATEGORIES_URL don't run concurrently.
    use std::sync::Mutex;

    use anyhow::Result;
    use url::Url;

    use super::*;
    use crate::EnvGuard;
    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    // Test successful retrieval of the TOML string.
    #[test]
    fn test_get_categories_toml_string_success() -> Result<()> {
        let _lock = ENV_MUTEX.lock().unwrap();
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
        let _url_guard = EnvGuard::set("MOCK_CATEGORIES_URL", &mock_url);

        // Call the function under test.
        let result = get_categories_toml_string();

        // Assertions
        mock_endpoint.assert(); // Verify the mock endpoint was hit.
        assert!(
            result.is_ok(),
            "Expected Ok result, got Err: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap(), expected_toml);

        Ok(())
    }

    // Test handling of a network/server error (e.g., 404 Not Found).
    #[test]
    fn test_get_categories_toml_string_error() -> Result<()> {
        let _lock = ENV_MUTEX.lock().unwrap();
        let mut server = mockito::Server::new();

        // Mock the GET request to return a 404 error.
        let mock_endpoint = server
            .mock("GET", "/")
            .with_status(404)
            .with_body("Not Found")
            .create();

        let mock_url = format!("{}/", server.url());
        let _url_guard = EnvGuard::set("MOCK_CATEGORIES_URL", &mock_url);

        // Call the function under test.
        let result = get_categories_toml_string();

        // Assertions.
        mock_endpoint.assert(); // Verify the mock endpoint was hit.
        assert!(result.is_err(), "Expected Err result, got Ok");

        // Optionally, check the specific error kind or message if needed.
        let err_string = result.unwrap_err().to_string();
        assert!(
            err_string.contains("404") || err_string.contains("Not Found"),
            "Error message did not indicate a 404 error: {err_string}"
        );

        Ok(())
    }
}
