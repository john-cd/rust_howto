//! Retrieves crate metadata and retains contextual lookup failures.
//!
//! TODO Reviewed ??-??-2026.

use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::Mutex;

use anyhow::Context as _;
use anyhow::Result;
use crates_io_api::CrateResponse;
use tracing::debug;

/// Caches crate info responses retrieved from crates.io.
pub(super) static CRATE_INFO_CACHE: LazyLock<Mutex<HashMap<String, CrateResponse>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Returns information for a crate, given its name
///
/// # Errors
/// Returns client initialization or crates.io lookup failures with the requested crate name.
pub fn get_info_for_crate(crate_name: &str) -> Result<CrateResponse> {
    if let Ok(cache) = CRATE_INFO_CACHE.lock()
        && let Some(info) = cache.get(crate_name)
    {
        debug!("Cache hit for crates.io crate info lookup of {crate_name}");
        return Ok(info.clone());
    }

    let client = super::get_client()?;
    let info = get_info_for_crate_with(crate_name, |name| Ok(client.get_crate(name)?))?;

    if let Ok(mut cache) = CRATE_INFO_CACHE.lock() {
        cache.insert(crate_name.to_owned(), info.clone());
    }

    Ok(info)
}

/// Clears the in-memory crate info cache.
pub fn clear_crate_info_cache() {
    if let Ok(mut cache) = CRATE_INFO_CACHE.lock() {
        cache.clear();
    }
}

/// Fetches metadata with request logging and crate-specific error context.
fn get_info_for_crate_with(
    crate_name: &str,
    fetch: impl FnOnce(&str) -> Result<CrateResponse>,
) -> Result<CrateResponse> {
    debug!("Calling crates.io API for {crate_name}");
    fetch(crate_name)
        .with_context(|| format!("Failed to retrieve metadata for crate `{crate_name}`"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Sample crates.io API response used by the request parsing tests.
    const CRATE_RESPONSE: &str = r#"{
        "crate": {
            "id": "fixture-crate",
            "name": "fixture-crate",
            "description": "Fixture response",
            "documentation": null,
            "homepage": null,
            "repository": null,
            "downloads": 1,
            "recent_downloads": 1,
            "categories": [],
            "keywords": [],
            "versions": [],
            "max_version": "1.0.0",
            "max_stable_version": "1.0.0",
            "links": {
                "owner_team": "/api/v1/crates/fixture-crate/owner_team",
                "owner_user": "/api/v1/crates/fixture-crate/owner_user",
                "owners": "/api/v1/crates/fixture-crate/owners",
                "reverse_dependencies": "/api/v1/crates/fixture-crate/reverse_dependencies",
                "version_downloads": "/api/v1/crates/fixture-crate/downloads",
                "versions": "/api/v1/crates/fixture-crate/versions"
            },
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z",
            "exact_match": true
        },
        "versions": [],
        "keywords": [],
        "categories": []
    }"#;

    /// Metadata lookup passes the requested name and preserves the response.
    #[test]
    fn returns_the_response_for_the_requested_crate() -> Result<()> {
        let response = get_info_for_crate_with("fixture-crate", |name| {
            assert_eq!(name, "fixture-crate");
            Ok(serde_json::from_str(CRATE_RESPONSE)?)
        })?;

        assert_eq!(response.crate_data.id, "fixture-crate");
        assert_eq!(
            response.crate_data.description.as_deref(),
            Some("Fixture response")
        );
        Ok(())
    }

    /// Lookup failures identify the requested crate and retain the underlying error.
    #[test]
    fn propagates_fetch_errors() {
        let error =
            get_info_for_crate_with("missing", |_| Err(anyhow::anyhow!("crate lookup failed")))
                .expect_err("fetch errors should be returned to the caller");

        assert_eq!(
            error.to_string(),
            "Failed to retrieve metadata for crate `missing`"
        );
        assert_eq!(error.root_cause().to_string(), "crate lookup failed");
    }

    /// Serializes tests that share the local cache.
    static TEST_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// Verifies that cached crate info results are returned and can be cleared.
    #[test]
    fn test_crate_info_cache_hit_and_clear() -> Result<()> {
        let _guard = TEST_MUTEX
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        clear_crate_info_cache();

        let fake_crate = "mock-crate-for-info-cache-test";
        let fake_response: CrateResponse = serde_json::from_str(CRATE_RESPONSE)?;

        // Seed internal cache
        if let Ok(mut cache) = CRATE_INFO_CACHE.lock() {
            cache.insert(fake_crate.to_owned(), fake_response.clone());
        }

        // Subsequent lookup hits the cache directly
        let result = get_info_for_crate(fake_crate).expect("cache hit should succeed");
        assert_eq!(result.crate_data.id, "fixture-crate");

        // Clear cache
        clear_crate_info_cache();
        if let Ok(cache) = CRATE_INFO_CACHE.lock() {
            assert!(cache.get(fake_crate).is_none());
        }
        Ok(())
    }

    /// Verifies that repeated cached crate info lookups remain within the expected time budget.
    #[test]
    fn test_crate_info_cache_performance() -> Result<()> {
        let _guard = TEST_MUTEX
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        clear_crate_info_cache();

        let fake_crate = "perf-test-crate-info";
        let fake_response: CrateResponse = serde_json::from_str(CRATE_RESPONSE)?;

        if let Ok(mut cache) = CRATE_INFO_CACHE.lock() {
            cache.insert(fake_crate.to_owned(), fake_response);
        }

        let start = std::time::Instant::now();
        let iterations = 1_000;
        for _ in 0..iterations {
            let res = get_info_for_crate(fake_crate)?;
            assert_eq!(res.crate_data.id, "fixture-crate");
        }
        let elapsed = start.elapsed();

        // 1000 cached lookups should execute in < 50ms
        assert!(
            elapsed.as_millis() < 50,
            "1000 cached crate info lookups took too long: {elapsed:?}"
        );

        clear_crate_info_cache();
        Ok(())
    }
}
