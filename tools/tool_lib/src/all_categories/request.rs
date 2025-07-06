use anyhow::Result;

// The master list of categories is a TOML file in the `crates.io` repo:
static CATEGORIES_URL: &str = "https://raw.githubusercontent.com/rust-lang/crates.io/refs/heads/main/src/boot/categories.toml";

pub(super) fn get_categories_toml_string() -> Result<String> {
    let response = reqwest::blocking::get(CATEGORIES_URL)?;
    let body = response.text()?;
    Ok(body)
}

#[allow(dead_code)]
#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use anyhow::Result;
    use url::Url;

    use super::*; // To parse the CATEGORIES_URL.
    // use crate::EnvGuard;

    // // FIXME client makes HTTPS requests, not HTTP
    // // Test successful retrieval of the TOML string.
    // #[test]
    // fn test_get_categories_toml_string_success() -> Result<()> {
    //     let mut server = mockito::Server::new();
    //     let expected_toml = "[category1]\nname = \"Category One\"\ndescription = \"First category\"\n\n[category2]\nname = \"Category Two\"\n";

    //     // Parse the hardcoded URL to get the path for mocking.
    //     let url = Url::parse(CATEGORIES_URL)?;
    //     let path = url.path(); // e.g., "/rust-lang/crates.io/..."

    //     // Mock the GET request to the specific path on the mock server
    //     let mock_endpoint = server
    //         .mock("GET", path)
    //         .with_status(200)
    //         .with_header("content-type", "text/plain; charset=utf-8") // Match typical GitHub raw response.
    //         .with_body(expected_toml)
    //         .create();

    //     // Set the HTTPS_PROXY environment variable to redirect the request to the mock server.
    //     // `EnvGuard` ensures the variable is restored when it goes out of scope.
    //     let _proxy_guard = EnvGuard::set("HTTPS_PROXY", &server.url());

    //     // Call the function under test.
    //     let result = get_categories_toml_string();

    //     // Assertions
    //     mock_endpoint.assert(); // Verify the mock endpoint was hit.
    //     assert!(result.is_ok(), "Expected Ok result, got Err: {:?}", result.err());
    //     assert_eq!(result.unwrap(), expected_toml);

    //     Ok(())
    // }

    //     // Test handling of a network/server error (e.g., 404 Not Found).
    //     #[test]
    //     fn test_get_categories_toml_string_error() -> Result<()> {
    //         let mut server = mockito::Server::new();

    //         // Parse the hardcoded URL to get the path.
    //         let url = Url::parse(CATEGORIES_URL)?;
    //         let path = url.path();

    //         // Mock the GET request to return a 404 error.
    //         let mock_endpoint = server
    //             .mock("GET", path)
    //             .with_status(404)
    //             .with_body("Not Found")
    //             .create();

    //         // Set the HTTPS_PROXY environment variable.
    //         let _proxy_guard = EnvGuard::set("HTTPS_PROXY", &server.url());

    //         // Call the function under test.
    //         let result = get_categories_toml_string();

    //         // Assertions.
    //         mock_endpoint.assert(); // Verify the mock endpoint was hit.
    //         assert!(result.is_err(), "Expected Err result, got Ok");

    //         // Optionally, check the specific error kind or message if needed.
    //         let err_string = result.unwrap_err().to_string();
    //         assert!(
    //             err_string.contains("404") && err_string.contains("Not Found"),
    //             "Error message did not indicate a 404 error: {}",
    //             err_string
    //         );

    //         Ok(())
    //     }
}
