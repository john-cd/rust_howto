<!-- TODO Reviewed ??-??-2026 -->

# Metrics

{{#include metrics.incl.md}}

Metrics software is used to measure, analyze, and improve various aspects of software performance. It can be used for debugging, performance optimization, cost estimation, and quality assurance.

Examples include request totals, in-flight work, latency distributions, cache hit ratios, and build durations. Metrics aggregate measurements and help answer how often or how long; use logs to inspect an individual event and traces to follow one request through a system.

## Serve Metrics from an Axum Route {#serve-metrics-from-an-axum-route}

Most services expose the payload on a `/metrics` route that Prometheus scrapes. The handler only calls `handle.render()`; the test drives the router with `tower::ServiceExt::oneshot`, so no port is opened.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/metrics/metrics_route.rs:example}}
```

## Naming and Label Cardinality {#naming-and-label-cardinality .skip}

- Every distinct label-value combination is a separate time series that the exporter keeps in memory. Use bounded values (HTTP method, route template, status class), never user IDs, URLs with parameters or error messages.
- Follow the Prometheus naming conventions: `snake_case`, a unit suffix (`_seconds`, `_bytes`), and `_total` for counters.
- The [`prometheus`][c~prometheus~docs]↗ crate is an alternative, lower-level API that owns its registry; `metrics` is a facade that lets libraries stay exporter-agnostic.
- Metrics aggregate, so they cannot say which request was slow. Pair them with [[tracing | tracing]] spans, which keep per-request detail.

## Useful Crates {#useful-crates .skip}

- [`metrics`][metrics~website]↗ is a lightweight instrumentation facade for libraries and applications. It supports counters, gauges, and histograms; an application chooses the recorder or exporter.
- [`metrics-exporter-prometheus`][metrics-exporter-prometheus~website]↗ exposes `metrics` measurements in Prometheus exposition format.
- [`tokio-metrics`][c~tokio-metrics~crates.io]↗ provides utilities for collecting metrics from a Tokio application, including runtime and per-task metrics.
- [`cadence`][c~cadence~crates.io]↗ is an extensible [`statsd`][statsd~repo]↗{{hi:statsd}} client for Rust.
- [`sentry`][c~sentry~crates.io]↗ is a [Sentry][getsentry~website]↗ client for Rust error reporting and monitoring.

## Related Topics {#related-topics .skip}

- [[distributed_telemetry | Distributed Telemetry]].
- [[tracing | Tracing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
