use anyhow::Result;
use tracing::warn;

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
    warn!("Calling crates.io API for {crate_name}");
    let crt = client.get_crate(crate_name)?;
    // println!("Categories: {:?}", crt.categories);
    Ok(crt
        .categories
        .into_iter()
        .map(model::Category::new)
        .collect())
}
