// TODO Reviewed ??-??-2026
use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::Mutex;

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
    if let Ok(cache) = CATEGORY_CACHE.lock()
        && let Some(cats) = cache.get(crate_name)
    {
        debug!("Cache hit for crates.io category lookup of {crate_name}");
        return Ok(cats.clone());
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
        cache.insert(crate_name.to_owned(), categories.clone());
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
    use anyhow::Result;
    use chrono::Utc;
    use crates_io_api::Category as CratesIoCategory;

    use crate::model;

    /// Creates a crates.io category fixture for cache and conversion tests.
    fn mock_crates_io_category(id: &str, name: &str, desc: &str) -> CratesIoCategory {
        CratesIoCategory {
            category: name.to_owned(),
            crates_cnt: 1,          // Dummy value.
            created_at: Utc::now(), // Dummy value.
            description: desc.to_owned(),
            id: id.to_owned(),
            slug: id.to_owned(),
        }
    }

    /// Verifies that the API category fields are copied into the local model.
    #[test]
    fn test_category_new() {
        let api_cat = mock_crates_io_category("web", "Web Stuff", "All about web");

        let model_cat = model::Category::new(api_cat.clone());

        assert_eq!(model_cat.category, api_cat.category);
        assert_eq!(model_cat.slug, api_cat.slug);
        assert_eq!(model_cat.description, api_cat.description);
    }

    static TEST_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// Verifies that cached category results are returned and can be cleared.
    #[test]
    fn test_category_cache_hit_and_clear() {
        let _guard = TEST_MUTEX
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        super::clear_category_cache();

        let fake_crate = "mock-crate-for-cache-test";
        let fake_categories = vec![model::Category {
            category: "Testing".to_owned(),
            slug: "testing".to_owned(),
            description: "Testing category".to_owned(),
        }];

        // Seed internal cache
        if let Ok(mut cache) = super::CATEGORY_CACHE.lock() {
            cache.insert(fake_crate.to_owned(), fake_categories.clone());
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

    /// Verifies that repeated cached lookups remain within the expected time budget.
    #[test]
    fn test_category_cache_performance() -> Result<()> {
        let _guard = TEST_MUTEX
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        super::clear_category_cache();

        let fake_crate = "perf-test-crate";
        let fake_categories = vec![model::Category {
            category: "Performance".to_owned(),
            slug: "performance".to_owned(),
            description: "Performance category".to_owned(),
        }];

        if let Ok(mut cache) = super::CATEGORY_CACHE.lock() {
            cache.insert(fake_crate.to_owned(), fake_categories);
        }

        let start = std::time::Instant::now();
        let iterations = 1_000;
        for _ in 0..iterations {
            let res = super::get_categories_for_crate(fake_crate)?;
            assert_eq!(res.len(), 1);
        }
        let elapsed = start.elapsed();

        // 1000 cached lookups should execute in < 50ms (typically < 1ms)
        assert!(
            elapsed.as_millis() < 50,
            "1000 cached lookups took too long: {elapsed:?}"
        );

        super::clear_category_cache();
        Ok(())
    }
}
