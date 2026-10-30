<!-- TODO Reviewed ??-??-2026 -->

# Distributed Observability and Telemetry

{{#include distributed_telemetry.incl.md}}

## Export Traces with OpenTelemetry and OTLP {#open-telemetry}

[![opentelemetry][c~opentelemetry~docs~badge]][c~opentelemetry~docs] [![opentelemetry~crates.io][c~opentelemetry~crates.io~badge]][c~opentelemetry~crates.io] [![opentelemetry~repo][c~opentelemetry~repo~badge]][c~opentelemetry~repo] [![opentelemetry~lib.rs][c~opentelemetry~lib.rs~badge]][c~opentelemetry~lib.rs]{{hi:opentelemetry}}{{hi:Logging}}{{hi:Metrics}}{{hi:opentelemetry}}{{hi:Tracing}} [![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}} [![cat~development-tools::profiling][cat~development-tools::profiling~badge]][cat~development-tools::profiling]{{hi:Profiling}}
[![opentelemetry-semantic-conventions][c~opentelemetry-semantic-conventions~docs~badge]][c~opentelemetry-semantic-conventions~docs]{{hi:opentelemetry-semantic-conventions}} [![opentelemetry-semantic-conventions~crates.io][c~opentelemetry-semantic-conventions~crates.io~badge]][c~opentelemetry-semantic-conventions~crates.io]

[OpenTelemetry][opentelemetry~website]↗ is a vendor-neutral API, SDK, and tooling ecosystem for generating and exporting telemetry. For distributed traces, instrument application code with `tracing` and connect it to the OpenTelemetry SDK through [`tracing-opentelemetry`][tracing-opentelemetry~docs]. The [`opentelemetry-otlp`][opentelemetry-otlp~docs] exporter sends spans in OTLP over gRPC or HTTP to a collector or backend. Set a stable `service.name` resource attribute, batch exports, and shut down the provider cleanly so buffered spans can be flushed. See the [official Rust OTLP example][opentelemetry-otlp-example] for a complete exporter setup.

The example below installs an SDK tracer provider with a stdout exporter, records a span, and shuts the provider down so the span is flushed. For an OTLP exporter to a collector or backend, follow the [official Rust OTLP example][opentelemetry-otlp-example].

The span attribute key comes from [`opentelemetry-semantic-conventions`][c~opentelemetry-semantic-conventions~docs]↗{{hi:opentelemetry-semantic-conventions}}, which provides constants such as `HTTP_ROUTE` for the standard attribute names. Using them, rather than retyping strings like `"http.route"`, keeps your telemetry consistent with other services and lets backends recognize the attributes. Only the stable conventions are available by default; the crate's `semconv_experimental` feature adds the rest.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/other/open_telemetry.rs:example}}
```

### Useful Links {#useful-links .skip}

- [OpenTelemetry Rust components and examples][opentelemetry-rust~repo].
- [OpenTelemetry OTLP exporter example][opentelemetry-otlp-example].

## Propagate Trace Context Across HTTP {#propagate-context-across-http}

Distributed traces need context propagation to connect spans across process boundaries. The W3C Trace Context format carries trace and parent span identifiers in `traceparent` (and optional vendor state in `tracestate`) HTTP headers. Framework instrumentation often injects and extracts these headers for you; when adapting an HTTP stack directly, [`opentelemetry-http`][opentelemetry-http~docs] provides `HeaderInjector` and `HeaderExtractor` adapters for an `http::HeaderMap` and OpenTelemetry's propagator API. Use the [context propagation guide][otel-context-propagation] to understand the request flow.

The example below injects the current span's context into a header map on the client side, extracts it in the handler, and parents the handler span on it. An in-memory span exporter lets the tests assert that both spans share a trace ID, that the handler is the client's child, and that a missing or malformed `traceparent` starts a new, independent trace. Treat incoming headers as untrusted: use only the `TraceContextPropagator` (no baggage propagator), never log header values, and attach the extracted context to the handler's own span rather than to the thread so concurrent requests cannot leak context into each other. Run the `trace_context` example with the `opentelemetry` feature enabled.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/other/trace_context.rs:example}}
```

## Prometheus Metrics {#prometheus}

Prometheus is commonly used for metrics collection. See [[metrics | Metrics]] for Rust counters, histograms, and a Prometheus exporter example; this chapter focuses on distributed traces.

### Useful Links {#useful-links2 .skip}

- [Grafana][grafana~website]↗.
- [Prometheus][prometheus~repo] monitoring system and time series database.
- [Thanos][thanos~website]↗: open source, highly available Prometheus setup with long term storage capabilities.

## `OpenObserve` {#open-observe}

[OpenObserve][openobserve~repo]↗{{hi:OpenObserve}} accepts OpenTelemetry traces over OTLP/HTTP. Set `OTEL_EXPORTER_OTLP_ENDPOINT` to the organization API base URL, such as `http://localhost:5080/api/default`; the exporter appends the OTLP trace path. Set `OTEL_EXPORTER_OTLP_HEADERS` with the authorization header required by your OpenObserve deployment. The example sends a span when an endpoint is configured and prints a setup hint otherwise. See the [trace-ingestion documentation][openobserve~traces-docs] for endpoint and authentication details.

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
