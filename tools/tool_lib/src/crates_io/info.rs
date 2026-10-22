//! Retrieves crate metadata and retains contextual lookup failures.
//!
//! TODO Reviewed ??-??-2026

use anyhow::Context as _;
use anyhow::Result;
use crates_io_api::CrateResponse;
use tracing::debug;

/// Returns information for a crate, given its name
///
/// # Errors
/// Returns client initialization or crates.io lookup failures with the requested crate name.
pub fn get_info_for_crate(crate_name: &str) -> Result<CrateResponse> {
    let client = super::get_client()?;
    get_info_for_crate_with(crate_name, |name| Ok(client.get_crate(name)?))
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
}
