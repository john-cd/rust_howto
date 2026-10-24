<!-- TODO Reviewed ??-??-2026 -->

# Distributed Observability and Telemetry

{{#include distributed_telemetry.incl.md}}

## Export Traces with OpenTelemetry and OTLP {#open-telemetry}

[![opentelemetry][c~opentelemetry~docs~badge]][c~opentelemetry~docs] [![opentelemetry~crates.io][c~opentelemetry~crates.io~badge]][c~opentelemetry~crates.io] [![opentelemetry~repo][c~opentelemetry~repo~badge]][c~opentelemetry~repo] [![opentelemetry~lib.rs][c~opentelemetry~lib.rs~badge]][c~opentelemetry~lib.rs]{{hi:opentelemetry}}{{hi:Logging}}{{hi:Metrics}}{{hi:opentelemetry}}{{hi:Tracing}} [![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}} [![cat~development-tools::profiling][cat~development-tools::profiling~badge]][cat~development-tools::profiling]{{hi:Profiling}}

[OpenTelemetry][opentelemetry~website]↗ is a vendor-neutral API, SDK, and tooling ecosystem for generating and exporting telemetry. For distributed traces, instrument application code with `tracing` and connect it to the OpenTelemetry SDK through [`tracing-opentelemetry`][tracing-opentelemetry~docs]. The [`opentelemetry-otlp`][opentelemetry-otlp~docs] exporter sends spans in OTLP over gRPC or HTTP to a collector or backend. Set a stable `service.name` resource attribute, batch exports, and shut down the provider cleanly so buffered spans can be flushed. See the [official Rust OTLP example][opentelemetry-otlp-example] for a complete exporter setup.

The example below creates an OpenTelemetry span through the API. Exporting it requires an SDK tracer provider and an exporter such as OTLP; the global tracer is a no-op until a provider is installed.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/other/open_telemetry.rs:example}}
```

### Useful Links {#useful-links .skip}

- [OpenTelemetry Rust components and examples][opentelemetry-rust~repo].
- [OpenTelemetry OTLP exporter example][opentelemetry-otlp-example].

## Propagate Trace Context Across HTTP {#propagate-context-across-http}

Distributed traces need context propagation to connect spans across process boundaries. The W3C Trace Context format carries trace and parent span identifiers in `traceparent` (and optional vendor state in `tracestate`) HTTP headers. Framework instrumentation often injects and extracts these headers for you; when adapting an HTTP stack directly, [`opentelemetry-http`][opentelemetry-http~docs] provides `HeaderInjector` and `HeaderExtractor` adapters for an `http::HeaderMap` and OpenTelemetry's propagator API. Use the [context propagation guide][otel-context-propagation] to understand the request flow.

## Prometheus Metrics {#prometheus}

Prometheus is commonly used for metrics collection. See [[metrics | Metrics]] for Rust counters, histograms, and a Prometheus exporter example; this chapter focuses on distributed traces.

### Useful Links {#useful-links2 .skip}

- [Grafana][grafana~website]↗.
- [Prometheus][prometheus~repo] monitoring system and time series database.
- [Thanos][thanos~website]↗: open source, highly available Prometheus setup with long term storage capabilities.

## `OpenObserve` {#open-observe}

[OpenObserve][openobserve~repo]↗{{hi:OpenObserve}} accepts OpenTelemetry traces over OTLP. The existing example shows endpoint configuration and local `tracing` fields; configure an OTLP exporter to send spans to the endpoint, using the [trace-ingestion documentation][openobserve~traces-docs] for its path and authentication requirements.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/other/open_observe.rs:example}}
```

## OTLP Backends {#other-common-observability-tools-platforms .skip}

OTLP separates application instrumentation from storage and query tools. These backends are a short starting list; their supported transports and deployment needs differ:

- [Jaeger][jaeger~repo]↗ receives traces in OTLP and provides a trace query UI. See the [Jaeger feature guide][jaeger~otlp-docs].
- [OpenObserve][openobserve~repo]↗ accepts OTLP traces and combines trace, log, and metric views; see its [trace-ingestion API][openobserve~traces-docs].
- [Quickwit][quickwit~repo]↗ indexes OTLP traces for search and can work with the Jaeger UI; see its [distributed tracing guide][quickwit~traces-docs].

## Related Topics {#related-topics .skip}

- [[metrics | Metrics]].
- [[tracing | Tracing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
