//! Getting the list of categories of `crates.io`.
//!
//! TODO Reviewed ??-??-2026

mod categories_from_toml;
mod request;

use anyhow::Context as _;
use anyhow::Result;

use super::Category;

/// Fetches and parses the categories TOML file from `crates.io`.
///
/// This function retrieves the TOML data, parses it, and then
/// extracts the category information into a vector of `Category` structs.
/// Returns a list of all categories that exist on `crates.io`.
///
/// # Errors
///
/// Returns an error if the request fails, the response is not valid TOML, or a category entry is
/// invalid.
pub fn get_all_categories() -> Result<Vec<Category>> {
    let toml_string = request::get_categories_toml_string()?;
    let toml: toml::value::Table =
        toml::from_str(&toml_string).context("Could not parse categories toml")?;

    let categories = categories_from_toml::categories_from_toml(&toml, None)?;
    Ok(categories)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;
    use crate::ENV_MUTEX;
    use crate::EnvGuard;

    /// Verifies that all categories are fetched and parsed from a successful response.
    #[test]
    fn test_get_all_categories_success() -> Result<()> {
        let _lock = ENV_MUTEX
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut server = mockito::Server::new();

        let sample_toml = r#"
[algorithms]
name = "Algorithms"
description = "Rust algorithms"

[algorithms.categories.sorting]
name = "Sorting"
description = "Sorting algorithms"
"#;

        let mock_endpoint = server
            .mock("GET", "/")
            .with_status(200)
            .with_header("content-type", "text/plain; charset=utf-8")
            .with_body(sample_toml)
            .create();

        let mock_url = format!("{}/", server.url());
        let _url_guard = EnvGuard::set("MOCK_CATEGORIES_URL", &mock_url);

        let categories = get_all_categories()?;
        mock_endpoint.assert();

        assert_eq!(categories.len(), 2);
        let algorithms = categories
            .iter()
            .find(|c| c.slug == "algorithms")
            .expect("the Algorithms category should be parsed");
        assert_eq!(algorithms.category, "Algorithms");
        assert_eq!(algorithms.description, "Rust algorithms");

        let sorting = categories
            .iter()
            .find(|c| c.slug == "algorithms::sorting")
            .expect("the Sorting subcategory should be parsed");
        assert_eq!(sorting.category, "Algorithms::Sorting");
        assert_eq!(sorting.description, "Sorting algorithms");

        Ok(())
    }

    /// Verifies that an HTTP error from the category source is returned to the caller.
    #[test]
    fn test_get_all_categories_request_error() -> Result<()> {
        let _lock = ENV_MUTEX
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut server = mockito::Server::new();

        let mock_endpoint = server
            .mock("GET", "/")
            .with_status(500)
            .with_body("Internal Server Error")
            .create();

        let mock_url = format!("{}/", server.url());
        let _url_guard = EnvGuard::set("MOCK_CATEGORIES_URL", &mock_url);

        let error = get_all_categories().expect_err("a 500 response should fail the request");
        mock_endpoint.assert();

        let err_str = error.to_string();
        assert!(err_str.contains("500") || err_str.contains("Request failed"));

        Ok(())
    }

    /// Verifies that malformed category TOML is reported as a parse error.
    #[test]
    fn test_get_all_categories_invalid_toml() -> Result<()> {
        let _lock = ENV_MUTEX
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut server = mockito::Server::new();

        let invalid_toml = "this is not valid toml = = = ";

        let mock_endpoint = server
            .mock("GET", "/")
            .with_status(200)
            .with_header("content-type", "text/plain; charset=utf-8")
            .with_body(invalid_toml)
            .create();

        let mock_url = format!("{}/", server.url());
        let _url_guard = EnvGuard::set("MOCK_CATEGORIES_URL", &mock_url);

        let error = get_all_categories().expect_err("malformed TOML should fail to parse");
        mock_endpoint.assert();

        let err_str = error.to_string();
        assert!(err_str.contains("Could not parse categories toml"));

        Ok(())
    }
}
