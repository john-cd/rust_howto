//! Measures request latency percentiles for a fixed set of observations.

// ANCHOR: example
use hdrhistogram::Histogram;

/// Reports p50, p95, and p99 before and after adding slow requests.
///
/// # Errors
///
/// Returns an error if the histogram cannot be created or a sample cannot be recorded.
fn main() -> anyhow::Result<()> {
    let mut latencies = Histogram::<u64>::new_with_bounds(1, 60_000, 3)?;
    for duration_ms in 10..=109 {
        latencies.record(duration_ms)?;
    }

    let before = [0.50, 0.95, 0.99].map(|q| latencies.value_at_quantile(q));
    for duration_ms in [800, 900, 1_000] {
        latencies.record(duration_ms)?;
    }
    let after = [0.50, 0.95, 0.99].map(|q| latencies.value_at_quantile(q));

    println!("p50, p95, p99 (ms): {before:?} -> {after:?}");
    assert!(after[0].abs_diff(before[0]) <= 2);
    assert!(after[2] >= 800 && before[2] < 110);
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Confirms a few slow requests raise p99 while p50 stays nearly stable.
    #[test]
    fn slow_requests_raise_the_tail() -> anyhow::Result<()> {
        super::main()
    }
}
