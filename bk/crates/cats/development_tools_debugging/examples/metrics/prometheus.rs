// ANCHOR: example
//! Renders metrics in Prometheus exposition format without opening a port.

use metrics::counter;
use metrics::histogram;
use metrics_exporter_prometheus::PrometheusBuilder;

/// Records sample measurements and renders their Prometheus payload.
fn main() -> anyhow::Result<()> {
    let recorder = PrometheusBuilder::new().build_recorder();
    let handle = recorder.handle();

    metrics::with_local_recorder(&recorder, || {
        counter!("http_requests_total", "method" => "GET").increment(1);
        histogram!("http_request_duration_seconds", "route" => "/health")
            .record(0.025);
    });

    println!("{}", handle.render());
    Ok(())
}
// ANCHOR_END: example

/// Runs the Prometheus exposition example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use metrics_exporter_prometheus::PrometheusBuilder;

    /// Verifies that the in-memory handle renders measurements without an HTTP
    /// scrape.
    #[test]
    fn renders_recorded_metrics() {
        let recorder = PrometheusBuilder::new().build_recorder();
        let handle = recorder.handle();

        metrics::with_local_recorder(&recorder, || {
            metrics::counter!("test_requests_total", "method" => "GET")
                .increment(1);
            metrics::histogram!("test_request_duration_seconds").record(0.025);
        });

        let rendered = handle.render();

        assert!(rendered.contains("test_requests_total"));
        assert!(rendered.contains("test_request_duration_seconds"));
    }
}
