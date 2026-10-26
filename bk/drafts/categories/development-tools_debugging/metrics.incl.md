<!-- TODO Reviewed ??-??-2026 -->

Metrics summarize numeric measurements over time. Use counters for totals, gauges for values that can rise or fall, and histograms for distributions such as request duration; use logs for individual events and traces for the path of a request across components. The [`metrics`][metrics~website] facade lets libraries emit measurements without choosing the application's recorder. Applications can install an exporter such as [`metrics-exporter-prometheus`][c~metrics-exporter-prometheus~docs] or bridge instrumentation into OpenTelemetry; see [[tracing | Tracing]] and [[distributed_telemetry | Distributed Telemetry]].

## Record Counters and Histograms {#record-counters-and-histograms}

Record a counter for each completed request and a histogram observation for its elapsed time. Add bounded labels such as the HTTP method or route template; avoid user IDs and other unbounded values.

{{#include ../../../crates/cats/development_tools_debugging/examples/metrics/record_metrics.rs:example}}

## Expose Prometheus Metrics {#expose-prometheus-metrics}

`metrics-exporter-prometheus` provides an HTTP scrape listener for applications. This example renders the same exposition payload through an in-memory handle, which is useful for tests and for applications that serve the payload through their own HTTP stack.

{{#include ../../../crates/cats/development_tools_debugging/examples/metrics/prometheus.rs:example}}

| Recipe                                                                           | Crates                                                               | Categories                                                                                                      |
| -------------------------------------------------------------------------------- | -------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| [Record Counters and Histograms][ex~development-tools_debugging~record-metrics]  | [`metrics`][metrics~website]                                         | [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging] |
| [Metrics with Prometheus][ex~development-tools_debugging~prometheus]             | [`metrics-exporter-prometheus`][metrics-exporter-prometheus~website] | [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging] |
| [Serve Metrics from an Axum Route][ex~development-tools_debugging~metrics-route] | [`metrics-exporter-prometheus`][metrics-exporter-prometheus~website] | [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging] |
