// ANCHOR: example
//! Serves the Prometheus payload from an Axum `/metrics` route.
//!
//! Metrics are numbers that describe a running service, such as request counts
//! and latencies. A monitoring system such as Prometheus scrapes them
//! periodically from an HTTP endpoint, conventionally `/metrics`.

use axum::Router;
use axum::routing::get;
use metrics_exporter_prometheus::PrometheusBuilder;

/// Builds a router whose `/metrics` route renders the recorder's snapshot.
fn router() -> (Router, metrics_exporter_prometheus::PrometheusRecorder) {
    let recorder = PrometheusBuilder::new().build_recorder();
    let handle = recorder.handle();
    let app = Router::new()
        .route("/metrics", get(move || async move { handle.render() }));
    (app, recorder)
}

/// Records a measurement and prints what a scrape would return.
fn main() -> anyhow::Result<()> {
    let (_app, recorder) = router();
    metrics::with_local_recorder(&recorder, || {
        metrics::counter!("http_requests_total", "method" => "GET")
            .increment(1);
    });
    // In a real service: `axum::serve(listener, app).await?;`
    println!("{}", recorder.handle().render());
    Ok(())
}
// ANCHOR_END: example

/// Runs the Axum `/metrics` route example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::Request;
    use axum::http::StatusCode;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::main;
    use super::router;

    /// Verifies that the `/metrics` route returns the recorded series.
    #[tokio::test]
    async fn metrics_route_renders_recorded_series() -> anyhow::Result<()> {
        let (app, recorder) = router();
        metrics::with_local_recorder(&recorder, || {
            metrics::counter!("route_test_total", "method" => "GET")
                .increment(1);
        });

        let response = app
            .oneshot(Request::get("/metrics").body(Body::empty())?)
            .await?;
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await?.to_bytes();
        assert!(String::from_utf8(body.to_vec())?.contains("route_test_total"));
        Ok(())
    }

    /// Exercises the user-facing example entry point.
    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
