use anyhow::Result;
use tracing::debug;

use crate::model;

// crates_io_api::Category is not Hash / Ord / Eq,
// thus we use a new struct.
// Otherwise, we could define a newtype and impl Ord,
// PartialOrd, PartialEq
// https://doc.rust-lang.org/stable/core/cmp/trait.Ord.html

impl model::Category {
    /// Creates a new `model::Category` from a `crates_io_api::Category`.
    pub(super) fn new(cat: crates_io_api::Category) -> Self {
        Self {
            category: cat.category,
            slug: cat.slug,
            description: cat.description,
        }
    }
}

/// Retrieves the categories associated with a given crate name from `crates.io`.
///
/// This function queries the `crates.io` API to fetch the category information for the specified crate.
///
/// Returns a list of categories for a crate, given its name
pub fn get_categories_for_crate(crate_name: &str) -> Result<Vec<model::Category>> {
    let client = super::get_client()?;
    debug!("Calling crates.io API for {crate_name}");
    let crt = client.get_crate(crate_name)?;
    // println!("Categories: {:?}", crt.categories);
    Ok(crt
        .categories
        .into_iter()
        .map(model::Category::new)
        .collect())
}

#[allow(dead_code)]
#[allow(unused_imports)]
#[cfg(test)]
mod tests {

    use anyhow::Result;
    use chrono::prelude::*;
    use crates_io_api::Category as CratesIoCategory;
    use crates_io_api::Crate;
    use crates_io_api::CrateLinks;
    use crates_io_api::CrateResponse;

    use super::*;
    use crate::model;

    // Helper function to create a mock CratesIoCategory.
    fn mock_crates_io_category(id: &str, name: &str, desc: &str) -> CratesIoCategory {
        CratesIoCategory {
            category: name.to_string(),
            crates_cnt: 1,          // Dummy value.
            created_at: Utc::now(), // Dummy value.
            description: desc.to_string(),
            id: id.to_string(),
            slug: id.to_string(),
        }
    }

    // Helper function to create a mock Crate data.
    #[allow(deprecated)]
    fn mock_crate_data(name: &str, category_slugs: Vec<String>) -> Crate {
        Crate {
            id: name.to_string(),
            name: name.to_string(),
            description: Some(format!("Description for {name}")),
            license: None,
            documentation: None,
            homepage: None,
            repository: None,
            downloads: 100,
            recent_downloads: Some(10),
            categories: Some(category_slugs),
            keywords: None,
            versions: None,
            max_version: "1.0.0".to_string(),
            max_stable_version: Some("1.0.0".to_string()),
            links: CrateLinks {
                owner_team: "".to_string(),
                owner_user: "".to_string(),
                owners: "".to_string(),
                reverse_dependencies: "".to_string(),
                version_downloads: "".to_string(),
                versions: None,
            },
            created_at: Utc::now(),
            updated_at: Utc::now(),
            exact_match: None,
        }
    }

    #[test]
    fn test_category_new() {
        let api_cat = mock_crates_io_category("web", "Web Stuff", "All about web");

        let model_cat = model::Category::new(api_cat.clone());

        assert_eq!(model_cat.category, api_cat.category);
        assert_eq!(model_cat.slug, api_cat.slug);
        assert_eq!(model_cat.description, api_cat.description);
    }

    // FIXME client makes HTTPS request, not HTTP.

    // #[test]
    // fn test_get_categories_for_crate_success() -> Result<()> {
    //     let opts = mockito::ServerOpts {  ..Default::default() }; // host: "127.0.0.1", port: 443,
    //     let mut server = mockito::Server::new_with_opts(opts);

    //     // Use one of these addresses to configure your client.
    //     let host = server.host_with_port();
    //     let url = server.url();
    //     println!("host: {}; url: {}", host, url);

    //     let crate_name = "test-crate";

    //     let cat1 = mock_crates_io_category("cat-1", "Category One", "Desc One");
    //     let cat2 = mock_crates_io_category("cat-2", "Category Two", "Desc Two");

    //     // Mock the CrateResponse structure that the API would return.
    //     let mock_response = CrateResponse {
    //         crate_data: mock_crate_data(crate_name, vec![cat1.slug.clone(), cat2.slug.clone()]),
    //         versions: vec![],
    //         keywords: vec![],
    //         categories: vec![cat1.clone(), cat2.clone()], // The actual category details.
    //     };

    //     // Mock the HTTP endpoint.
    //     let mock_endpoint = server
    //         .mock("GET", &*format!("/api/v1/crates/{}", crate_name))
    //         .with_status(200)
    //         .with_header("content-type", "application/json")
    //         .with_body(serde_json::to_string(&mock_response)?)
    //         .create();

    //     // Set HTTPS_PROXY env var so that ureq (used by crates_io_api) routes requests to the mock server.
    //     // Store original proxy value if it exists.
    //     let original_proxy = std::env::var("HTTPS_PROXY").ok();
    //     unsafe {
    //         std::env::set_var("HTTPS_PROXY", server.url());
    //     }

    //     // Call the function under test.
    //     let result = get_categories_for_crate(crate_name);

    //     // Restore original proxy value or remove the var.
    //     if let Some(proxy) = original_proxy {
    //         unsafe {
    //             std::env::set_var("HTTPS_PROXY", proxy);
    //         }
    //     } else {
    //         unsafe {
    //             std::env::remove_var("HTTPS_PROXY");
    //         }
    //     }

    //     // Assert the mock endpoint was called.
    //     mock_endpoint.assert();

    //     // Check the result.
    //     let categories = result?;
    //     assert_eq!(categories.len(), 2);
    //     assert_eq!(categories[0].category, cat1.category);
    //     assert_eq!(categories[0].slug, cat1.slug);
    //     assert_eq!(categories[0].description, cat1.description);
    //     assert_eq!(categories[1].category, cat2.category);
    //     assert_eq!(categories[1].slug, cat2.slug);
    //     assert_eq!(categories[1].description, cat2.description);

    //     Ok(())
    // }

    // #[test]
    // fn test_get_categories_for_crate_no_categories() -> Result<()> {
    //     let mut server = mockito::Server::new();
    //     let crate_name = "no-cats-crate";

    //     // Mock response with empty categories list.
    //     let mock_response = CrateResponse {
    //         crate_data: mock_crate_data(crate_name, vec![]), // No category slugs.
    //         versions: vec![],
    //         keywords: vec![],
    //         categories: vec![], // Empty category details.
    //     };

    //     let mock_endpoint = server
    //         .mock("GET", &*format!("/api/v1/crates/{}", crate_name))
    //         .with_status(200)
    //         .with_header("content-type", "application/json")
    //         .with_body(serde_json::to_string(&mock_response)?)
    //         .create();

    //     let original_proxy = std::env::var("HTTPS_PROXY").ok();
    //     unsafe {
    //         std::env::set_var("HTTPS_PROXY", server.url());
    //     }

    //     let result = get_categories_for_crate(crate_name);

    //     if let Some(proxy) = original_proxy {
    //         unsafe {
    //             std::env::set_var("HTTPS_PROXY", proxy);
    //         }
    //     } else {
    //         unsafe {
    //             std::env::remove_var("HTTPS_PROXY");
    //         }
    //     }

    //     mock_endpoint.assert();

    //     let categories = result?;
    //     assert!(categories.is_empty());

    //     Ok(())
    // }

    // #[test]
    // fn test_get_categories_for_crate_api_error() -> Result<()> {
    //     let mut server = mockito::Server::new();
    //     let crate_name = "error-crate";

    //     // Mock a 404 Not Found error
    //     let mock_endpoint = server
    //         .mock("GET", &*format!("/api/v1/crates/{}", crate_name))
    //         .with_status(404)
    //         .with_header("content-type", "application/json")
    //         .with_body(r#"{"errors": [{"detail": "Not Found"}]}"#) // Example error body
    //         .create();

    //     let original_proxy = std::env::var("HTTPS_PROXY").ok();
    //     unsafe {
    //         std::env::set_var("HTTPS_PROXY", server.url());
    //     }

    //     // Call the function and expect an error
    //     let result = get_categories_for_crate(crate_name);

    //     if let Some(proxy) = original_proxy {
    //         unsafe {
    //             std::env::set_var("HTTPS_PROXY", proxy);
    //         }
    //     } else {
    //         unsafe {
    //             std::env::remove_var("HTTPS_PROXY");
    //         }
    //     }

    //     mock_endpoint.assert();

    //     assert!(result.is_err());
    //     // Optionally check the specific error kind if needed
    //     // e.g., using downcast_ref or checking the error message
    //     eprintln!("Got expected error: {:?}", result.err().unwrap());

    //     Ok(())
    // }
}
