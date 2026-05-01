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
    Ok(crt
        .categories
        .into_iter()
        .map(model::Category::new)
        .collect())
}

#[cfg(test)]
mod tests {

    use chrono::Utc;
    use crates_io_api::Category as CratesIoCategory;

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

    #[test]
    fn test_category_new() {
        let api_cat = mock_crates_io_category("web", "Web Stuff", "All about web");

        let model_cat = model::Category::new(api_cat.clone());

        assert_eq!(model_cat.category, api_cat.category);
        assert_eq!(model_cat.slug, api_cat.slug);
        assert_eq!(model_cat.description, api_cat.description);
    }

}
