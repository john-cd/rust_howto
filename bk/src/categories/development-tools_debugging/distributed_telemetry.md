# Distributed Observability and Telemetry

{{#include distributed_telemetry.incl.md}}

Prometheus and OpenTelemetry are two prominent open-source observability projects under the Cloud Native Computing Foundation (CNCF).

## Distributed Tracing with `OpenTelemetry` {#open-telemetry}

`OpenTelemetry` is a complete observability framework suitable for monitoring microservices and "cloud-native", modern, distributed systems. It is compatible with most major OSS and commercial backends.

`OpenTelemetry` is a collection of APIs, SDKs, and tools to instrument, generate, collect, and export metrics, logs, and traces to help analyze your software's performance and behavior.

Distributed tracing, a key aspect of observability, enables developers to track and analyze requests spanning multiple services.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/tests/other/open_telemetry.rs:example}}
```

## `OpenObserve` {#open-observe}

[OpenObserve][openobserve-github]{{hi:OpenObserve}}⮳ (written in Rust) is a petabyte-scale Elasticsearch / Splunk / DataDog alternative for logs, metrics, traces, RUM, error tracking, and session replay.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/tests/other/open_observe.rs:example}}
```

## References {#skip}

- [OpenTelemetry Rust [documentation][p-documentation]][opentelemetry-rust]⮳.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write / expand; link to CNCF; cross link

https://opentelemetry.io/
https://github.com/open-telemetry
https://github.com/davidB/tracing-opentelemetry-instrumentation-sdk/tree/main/axum-tracing-opentelemetry
https://tirslo.medium.com/opentelemetry-examples-with-rust-67f77ccb025f

Review:

- Prometheus https://github.com/prometheus/prometheus  https://crates.io/crates/prometheus
- Zipkin
- Splunk
- DataDog
- Thanos https://thanos.io/
- Grafana
- Fluentd: Open-Source Log Collector https://github.com/fluent/fluentd

## Jaeger {#jaeger}

`Jaeger` is a Cloud Native Computing Foundation (CNCF) open-source, end-to-end distributed tracing system built for monitoring and troubleshooting microservices-based architectures.

Usually incorporated as an OpenTelemetry backend, it is used to track and visualize user request behavior across the distributed components of complex systems.

https://github.com/jaegertracing/jaeger

</div>
