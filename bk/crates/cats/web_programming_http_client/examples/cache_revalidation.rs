// TODO Reviewed ??-??-2026.
//! Models a fresh cache hit and an ETag-based revalidation without a network.

// ANCHOR: example
use std::time::Duration;
use std::time::UNIX_EPOCH;

use http::Request;
use http::Response;
use http::StatusCode;
use http::header::CACHE_CONTROL;
use http::header::ETAG;
use http::header::IF_NONE_MATCH;
use http_cache_semantics::AfterResponse;
use http_cache_semantics::BeforeRequest;
use http_cache_semantics::CacheOptions;
use http_cache_semantics::CachePolicy;

/// Checks reuse while fresh and conditional revalidation after expiry.
///
/// # Errors
///
/// Returns an error if synthetic HTTP messages cannot be built.
fn main() -> anyhow::Result<()> {
    const TAG: &str = r#""v1""#;
    let received_at = UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    let request = Request::get("https://example.com/data").body(())?;
    let response = Response::builder()
        .header(CACHE_CONTROL, "max-age=60")
        .header(ETAG, TAG)
        .body("cached body")?;
    let policy =
        CachePolicy::new_options(&request, &response, received_at, CacheOptions::default());
    assert!(policy.is_storable());
    assert!(matches!(
        policy.before_request(&request, received_at + Duration::from_secs(30)),
        BeforeRequest::Fresh(_)
    ));

    let expired_at = received_at + Duration::from_secs(61);
    let BeforeRequest::Stale {
        request: conditional,
        matches: true,
    } = policy.before_request(&request, expired_at)
    else {
        anyhow::bail!("expected a conditional request for the cached resource");
    };
    assert_eq!(conditional.headers[IF_NONE_MATCH], TAG);

    let not_modified = Response::builder()
        .status(StatusCode::NOT_MODIFIED)
        .header(ETAG, TAG)
        .body(())?;
    assert!(matches!(
        policy.after_response(&conditional, &not_modified, expired_at),
        AfterResponse::NotModified(_, _)
    ));
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Uses a fresh entry and keeps its body after an ETag validation.
    #[test]
    fn revalidates_an_expired_entry() -> anyhow::Result<()> {
        super::main()
    }
}
