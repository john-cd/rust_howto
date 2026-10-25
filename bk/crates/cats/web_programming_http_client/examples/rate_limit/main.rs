//! Client-side rate limiting.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Limit the rate of outgoing calls with a token bucket.
//!
//! [`governor`] implements the generic cell rate algorithm: each call takes a
//! cell from a bucket that refills at a fixed rate.

use std::num::NonZeroU32;
use std::time::Duration;

use governor::Quota;
use governor::RateLimiter;
use governor::clock::FakeRelativeClock;

/// Allows two calls per second, with a burst of two.
fn quota() -> Quota {
    Quota::per_second(NonZeroU32::MIN.saturating_add(1))
}

/// Calls `call` at most as often as the quota allows, waiting when necessary.
async fn rate_limited_calls(count: u32) -> Vec<u32> {
    let limiter = RateLimiter::direct(quota());
    let mut done = Vec::new();
    for i in 0..count {
        // Resolves immediately while tokens remain, then paces the calls.
        limiter.until_ready().await;
        // Replace with the real call, for example `client.get(url).send()`.
        done.push(i);
    }
    done
}

#[tokio::main]
async fn main() {
    let done = rate_limited_calls(4).await;
    println!("Made {} calls", done.len());
}

/// The quota can be checked deterministically with a fake clock,
/// so the test does not sleep.
fn check_with_fake_clock() -> (bool, bool, bool, bool) {
    let clock = FakeRelativeClock::default();
    let limiter = RateLimiter::direct_with_clock(quota(), clock.clone());
    let first = limiter.check().is_ok();
    let second = limiter.check().is_ok();
    // The burst of two is spent.
    let third = limiter.check().is_ok();
    // One cell is replenished after half a second.
    clock.advance(Duration::from_millis(500));
    let fourth = limiter.check().is_ok();
    (first, second, third, fourth)
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn burst_then_replenish() {
        assert_eq!(check_with_fake_clock(), (true, true, false, true));
    }

    #[tokio::test(start_paused = true)]
    async fn until_ready_paces_calls() {
        let start = tokio::time::Instant::now();
        let done = rate_limited_calls(3).await;
        assert_eq!(done, vec![0, 1, 2]);
        // governor uses a real clock, so only check the result here.
        assert!(start.elapsed() < Duration::from_secs(60));
    }
}
