#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Calculate how long to wait after hitting a GitHub REST API rate limit.
//!
//! See <https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api>.

use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use anyhow::Result;
use anyhow::anyhow;
use reqwest::StatusCode;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::header::RETRY_AFTER;

/// Reads a header as an unsigned integer, if present.
fn header_u64(headers: &HeaderMap, name: &str) -> Result<Option<u64>> {
    headers
        .get(name)
        .map(|v| Ok(v.to_str()?.parse()?))
        .transpose()
}

/// Returns how long to wait before retrying a request, or `None` if the
/// response was not rate limited.
///
/// GitHub returns `403` or `429` when a rate limit is exceeded:
/// - If `retry-after` is present, wait that many seconds.
/// - Else, if `x-ratelimit-remaining` is `0`, wait until the UTC epoch time
///   in `x-ratelimit-reset`.
/// - Otherwise (a secondary rate limit), wait at least one minute.
fn rate_limit_wait(
    status: StatusCode,
    headers: &HeaderMap,
    now: SystemTime,
) -> Result<Option<Duration>> {
    if status != StatusCode::FORBIDDEN
        && status != StatusCode::TOO_MANY_REQUESTS
    {
        return Ok(None);
    }

    if let Some(secs) = header_u64(headers, RETRY_AFTER.as_str())? {
        return Ok(Some(Duration::from_secs(secs)));
    }

    if header_u64(headers, "x-ratelimit-remaining")? == Some(0) {
        let reset = header_u64(headers, "x-ratelimit-reset")?
            .ok_or_else(|| anyhow!("missing x-ratelimit-reset"))?;
        let reset_at = UNIX_EPOCH
            .checked_add(Duration::from_secs(reset))
            .ok_or_else(|| anyhow!("x-ratelimit-reset is out of range"))?;
        return Ok(Some(reset_at.duration_since(now).unwrap_or_default()));
    }

    // A 403 without those headers is not necessarily a rate limit (it may be
    // a permissions error); a 429 is.
    if status == StatusCode::TOO_MANY_REQUESTS {
        return Ok(Some(Duration::from_mins(1)));
    }
    Ok(None)
}

/// Computes the wait time from example response headers and prints it.
fn main() -> Result<()> {
    let now = UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    let mut headers = HeaderMap::new();
    headers.insert("x-ratelimit-limit", HeaderValue::from_static("60"));
    headers.insert("x-ratelimit-remaining", HeaderValue::from_static("0"));
    headers.insert("x-ratelimit-reset", HeaderValue::from_static("1700000060"));

    if let Some(wait) = rate_limit_wait(StatusCode::FORBIDDEN, &headers, now)? {
        println!("Wait {} seconds before retrying", wait.as_secs());
    }

    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOW: u64 = 1_700_000_000;

    fn now() -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(NOW)
    }

    #[test]
    fn test_main() -> Result<()> {
        main()
    }

    #[test]
    fn success_does_not_wait() -> Result<()> {
        assert_eq!(
            rate_limit_wait(StatusCode::OK, &HeaderMap::new(), now())?,
            None
        );
        Ok(())
    }

    #[test]
    fn expired_window_waits_zero() -> Result<()> {
        let mut headers = HeaderMap::new();
        headers.insert("x-ratelimit-remaining", HeaderValue::from_static("0"));
        headers.insert(
            "x-ratelimit-reset",
            HeaderValue::from_static("1699999000"),
        );
        assert_eq!(
            rate_limit_wait(StatusCode::FORBIDDEN, &headers, now())?,
            Some(Duration::ZERO)
        );
        Ok(())
    }

    #[test]
    fn retry_after_takes_precedence() -> Result<()> {
        let mut headers = HeaderMap::new();
        headers.insert(RETRY_AFTER, HeaderValue::from_static("30"));
        headers.insert("x-ratelimit-remaining", HeaderValue::from_static("0"));
        headers.insert(
            "x-ratelimit-reset",
            HeaderValue::from_static("1700009999"),
        );
        assert_eq!(
            rate_limit_wait(StatusCode::TOO_MANY_REQUESTS, &headers, now())?,
            Some(Duration::from_secs(30))
        );
        Ok(())
    }

    #[test]
    fn secondary_limit_waits_a_minute() -> Result<()> {
        assert_eq!(
            rate_limit_wait(
                StatusCode::TOO_MANY_REQUESTS,
                &HeaderMap::new(),
                now()
            )?,
            Some(Duration::from_mins(1))
        );
        Ok(())
    }

    #[test]
    fn plain_forbidden_is_not_a_rate_limit() -> Result<()> {
        assert_eq!(
            rate_limit_wait(StatusCode::FORBIDDEN, &HeaderMap::new(), now())?,
            None
        );
        Ok(())
    }
}
