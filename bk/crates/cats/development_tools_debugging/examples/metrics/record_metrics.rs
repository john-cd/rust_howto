// ANCHOR: example
//! Records request counts and durations with the `metrics` facade.
//!
//! The `metrics` crate is a facade like `log`: code records counters, gauges
//! and histograms, and the application chooses an exporter that publishes them.
//! Keep label values bounded (for example the route, not the user ID) so the
//! number of series stays small.

use metrics::counter;
use metrics::histogram;

/// Records two representative HTTP request measurements.
fn main() -> anyhow::Result<()> {
    counter!("http_requests_total", "method" => "GET").increment(1);
    histogram!("http_request_duration_seconds", "route" => "/health")
        .record(0.025);

    println!("Recorded one request and a 25 ms duration.");
    Ok(())
}
// ANCHOR_END: example

/// Runs the metrics recording example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use metrics_util::debugging::DebuggingRecorder;

    use super::main;

    /// Verifies the facade records the counter and histogram into a local
    /// recorder.
    #[test]
    fn records_request_count_and_duration() {
        let recorder = DebuggingRecorder::new();
        let snapshotter = recorder.snapshotter();

        metrics::with_local_recorder(&recorder, || {
            main().expect("recording metrics should succeed");
        });

        let snapshot = snapshotter.snapshot();
        let values = snapshot.into_vec();

        assert!(values.iter().any(|(_, _, _, value)| {
            matches!(value, metrics_util::debugging::DebugValue::Counter(1))
        }));
        assert!(values.iter().any(|(_, _, _, value)| {
            matches!(value, metrics_util::debugging::DebugValue::Histogram(samples) if samples.iter().any(|sample| sample.into_inner() == 0.025))
        }));
    }
}
