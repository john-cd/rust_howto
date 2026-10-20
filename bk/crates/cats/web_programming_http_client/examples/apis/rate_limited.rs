#![allow(dead_code)]
// ANCHOR: example
//! Calculate how long an application should wait after hitting a rate limit.

use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use anyhow::Result;
use anyhow::anyhow;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;

fn parse_rate_limit_wait(
    headers: &HeaderMap,
    now: SystemTime,
) -> Result<Option<Duration>> {
    let limit: u64 = headers
        .get("x-ratelimit-limit")
        .ok_or_else(|| anyhow!("missing x-ratelimit-limit"))?
        .to_str()?
        .parse()?;
    let remaining: u64 = headers
        .get("x-ratelimit-remaining")
        .ok_or_else(|| anyhow!("missing x-ratelimit-remaining"))?
        .to_str()?
        .parse()?;
    let reset_epoch: u64 = headers
        .get("x-ratelimit-reset")
        .ok_or_else(|| anyhow!("missing x-ratelimit-reset"))?
        .to_str()?
        .parse()?;

    println!("Current limit window: {remaining}/{limit} remaining");

    if remaining == 0 {
        let reset_at = UNIX_EPOCH
            .checked_add(Duration::from_secs(reset_epoch))
            .ok_or_else(|| anyhow!("x-ratelimit-reset is out of range"))?;
        let wait = reset_at.duration_since(now).unwrap_or_default();
        return Ok(Some(wait));
    }

    Ok(None)
}

fn main() -> Result<()> {
    let now = UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    let mut headers = HeaderMap::new();
    headers.insert("x-ratelimit-limit", HeaderValue::from_static("60"));
    headers.insert("x-ratelimit-remaining", HeaderValue::from_static("0"));
    headers.insert("x-ratelimit-reset", HeaderValue::from_static("1700000060"));

    if let Some(wait) = parse_rate_limit_wait(&headers, now)? {
        println!("Wait {} seconds before retrying", wait.as_secs());
    }

    Ok(())
}
// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() -> Result<()> {
        main()
    }

    #[test]
    fn expired_rate_limit_window_does_not_require_waiting() -> Result<()> {
        let mut headers = HeaderMap::new();
        headers.insert("x-ratelimit-limit", HeaderValue::from_static("60"));
        headers.insert("x-ratelimit-remaining", HeaderValue::from_static("0"));
        headers.insert(
            "x-ratelimit-reset",
            HeaderValue::from_static("1700000000"),
        );

        let now = UNIX_EPOCH + Duration::from_secs(1_700_000_060);
        assert_eq!(parse_rate_limit_wait(&headers, now)?, Some(Duration::ZERO));
        Ok(())
    }
    // [fix interaction with https://docs.github.com/en/rest?apiVersion=2022-11-28](https://github.com/john-cd/rust_howto/issues/177)
}
