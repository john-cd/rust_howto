# Metrics

{{#include metrics.incl.md}}

Metrics software is used to measure, analyze, and improve various aspects of software performance. It can be used for debugging, performance optimization, cost estimation, and quality assurance.

Examples of common metrics include CPU and memory usage; cache hit ratios; response time, latency, throughput (queries per second) for APIs; build success rate and pipeline duration for CD/CI workflows.

## Useful Crates {#skip}

- [`metrics`](https://metrics.rs) is a batteries-included instrumentation ecosystem for Rust, allowing you to quickly and easily instrument your libraries and applications. It supports three fundamental metric types: counters, gauges, and histograms.
- [`tokio-metrics`](https://crates.io/crates/tokio-metrics) provides utilities for collecting metrics from a Tokio application, including runtime and per-task metrics.
- [`cadence`](https://crates.io/crates/cadence) is an extensible `statsd` client for Rust.
- [`sentry`](https://crates.io/crates/sentry) is a ['Sentry'](https://getsentry.com) client for Rust.

## Related Topics {#skip}

- [[distributed_telemetry | Distributed Telemetry]].
- [[tracing | Tracing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1345)
</div>
