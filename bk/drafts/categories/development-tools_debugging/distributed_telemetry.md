# Distributed Observability and Telemetry

{{#include distributed_telemetry.incl.md}}

Prometheus and OpenTelemetry are two prominent open-source observability projects under the Cloud Native Computing Foundation (CNCF).

## Distributed Tracing, Logs, and Metrics with `OpenTelemetry` {#open-telemetry}

[![opentelemetry][c~opentelemetry~docs~badge]][c~opentelemetry~docs] [![opentelemetry~crates.io][c~opentelemetry~crates.io~badge]][c~opentelemetry~crates.io] [![opentelemetry~github][c~opentelemetry~github~badge]][c~opentelemetry~github] [![opentelemetry~lib.rs][c~opentelemetry~lib.rs~badge]][c~opentelemetry~lib.rs]{{hi:opentelemetry}}{{hi:Logging}}{{hi:Metrics}}{{hi:opentelemetry}}{{hi:Tracing}} [![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}} [![cat~development-tools::profiling][cat~development-tools::profiling~badge]][cat~development-tools::profiling]{{hi:Profiling}}

[OpenTelemetry][opentelemetry~website][(GitHub)](https://github.com/open-telemetry)↗ is a complete observability framework suitable for monitoring microservices and "cloud-native", modern, distributed systems. It is compatible with most major OSS and commercial backends. [`OpenTelemetry`]( ){{hi: }} is a collection of APIs, SDKs, and tools to instrument, generate, collect, and export metrics, logs, and traces to help analyze your software's performance and behavior. Distributed tracing, a key aspect of observability, enables developers to track and analyze requests spanning multiple services.

The [`opentelemetry`]( ){{hi: }} crate is an OpenTelemetry API for Rust.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/other/open_telemetry.rs:example}}
```

### Useful Links {#skip}

- [`axum-tracing-opentelemetry` crate (GitHub)][tracing-axum-opentelemetry~github]↗.
- [OpenTelemetry with Rust - documentation][opentelemetry~rust]↗.
- [OpenTelemetry Examples with Rust][blog~tirslo.medium.com-opentelemetry~examples-with-rust]↗.
- [`Jaeger`][jaeger~github]↗ is a Cloud Native Computing Foundation (CNCF) open-source, end-to-end distributed tracing system built for monitoring and troubleshooting microservices-based architectures. Usually incorporated as an OpenTelemetry backend, it is used to track and visualize user request behavior across the distributed components of complex systems.
- [Zipkin][zipkin~website]↗ is a distributed tracing system.

## Metrics with Prometheus {#prometheus}

[![prometheus][c~prometheus~docs~badge]][c~prometheus~docs] [![prometheus~crates.io][c~prometheus~crates.io~badge]][c~prometheus~crates.io] [![prometheus~github][c~prometheus~github~badge]][c~prometheus~github] [![prometheus~lib.rs][c~prometheus~lib.rs~badge]][c~prometheus~lib.rs]{{hi:prometheus}}{{hi:Metrics}}{{hi:prometheus}}

Prometheus instrumentation library for Rust applications. Prometheus is an open-source systems monitoring and alerting toolkit. Prometheus's main features are:

- a multi-dimensional data model with time series data identified by metric name and key/value pairs
- PromQL, a flexible query language to leverage this dimensionality
- no reliance on distributed storage; single server nodes are autonomous
- time series collection happens via a pull model over HTTP
- pushing time series is supported via an intermediary gateway
- targets are discovered via service discovery or static configuration
- multiple modes of graphing and dashboarding support

### Useful Links {#skip}

- [Grafana][grafana~website].
- [Prometheus][prometheus~github] monitoring system and time series database.
- [Thanos][thanos~website]: open source, highly available Prometheus setup with long term storage capabilities.

## `OpenObserve` {#open-observe}

[OpenObserve][openobserve~github]{{hi:OpenObserve}}↗ (written in Rust) is a petabyte-scale Elasticsearch / Splunk / DataDog alternative for logs, metrics, traces, RUM (Real User Monitoring), error tracking, and session replay.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/other/open_observe.rs:example}}
```

## Other Common Observability Tools & Platforms {#skip}

- [Fluentd][fluentd~github] open-source log collector.
- [Splunk][splunk~website] is a unified security and observability platform.
- [DataDog][datadoghq~website].
- [Graphite][graphite-docs~website].
- [InfluxDB][influxdata~website].
- [Nagios][nagios~website].
- [New Relic][newrelic~website].

## Related Topics {#related-topics}

- [[metrics | Metrics]].
- [[tracing | Tracing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write / expand / organize. what to cover?](https://github.com/john-cd/rust_howto/issues/1343)

- [quickwit: Cloud-native search engine for observability. An open-source alternative to Datadog, Elasticsearch, Loki, and Tempo.][quickwit~github]

</div>
