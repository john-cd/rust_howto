// TODO Reviewed ??-??-2026
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use anyhow::Result;
use tracing::debug;

use crate::model;

static CATEGORY_CACHE: LazyLock<Mutex<HashMap<String, Vec<model::Category>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

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
    if let Ok(cache) = CATEGORY_CACHE.lock() {
        if let Some(cats) = cache.get(crate_name) {
            debug!("Cache hit for crates.io category lookup of {crate_name}");
            return Ok(cats.clone());
        }
    }

    let client = super::get_client()?;
    debug!("Calling crates.io API for {crate_name}");
    let crt = client.get_crate(crate_name)?;
    let categories: Vec<model::Category> = crt
        .categories
        .into_iter()
        .map(model::Category::new)
        .collect();

    if let Ok(mut cache) = CATEGORY_CACHE.lock() {
        cache.insert(crate_name.to_string(), categories.clone());
    }

    Ok(categories)
}

/// Clears the in-memory category cache.
pub fn clear_category_cache() {
    if let Ok(mut cache) = CATEGORY_CACHE.lock() {
        cache.clear();
    }
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

    static TEST_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn test_category_cache_hit_and_clear() {
        let _guard = TEST_MUTEX.lock().unwrap();
        super::clear_category_cache();

        let fake_crate = "mock-crate-for-cache-test";
        let fake_categories = vec![model::Category {
            category: "Testing".to_string(),
            slug: "testing".to_string(),
            description: "Testing category".to_string(),
        }];

        // Seed internal cache
        if let Ok(mut cache) = super::CATEGORY_CACHE.lock() {
            cache.insert(fake_crate.to_string(), fake_categories.clone());
        }

        // Subsequent lookup hits the cache directly
        let result = super::get_categories_for_crate(fake_crate).expect("cache hit should succeed");
        assert_eq!(result, fake_categories);

        // Clear cache
        super::clear_category_cache();
        if let Ok(cache) = super::CATEGORY_CACHE.lock() {
            assert!(cache.get(fake_crate).is_none());
        }
    }

    #[test]
    fn test_category_cache_performance() {
        let _guard = TEST_MUTEX.lock().unwrap();
        super::clear_category_cache();

        let fake_crate = "perf-test-crate";
        let fake_categories = vec![model::Category {
            category: "Performance".to_string(),
            slug: "performance".to_string(),
            description: "Performance category".to_string(),
        }];

        if let Ok(mut cache) = super::CATEGORY_CACHE.lock() {
            cache.insert(fake_crate.to_string(), fake_categories.clone());
        }

        let start = std::time::Instant::now();
        let iterations = 1_000;
        for _ in 0..iterations {
            let res = super::get_categories_for_crate(fake_crate).unwrap();
            assert_eq!(res.len(), 1);
        }
        let elapsed = start.elapsed();

        // 1000 cached lookups should execute in < 50ms (typically < 1ms)
        assert!(
            elapsed.as_millis() < 50,
            "1000 cached lookups took too long: {:?}",
            elapsed
        );

        super::clear_category_cache();
    }
}
