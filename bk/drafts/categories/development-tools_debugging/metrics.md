# Metrics

{{#include metrics.incl.md}}

Metrics software is used to measure, analyze, and improve various aspects of software performance. It can be used for debugging, performance optimization, cost estimation, and quality assurance.

Examples of common metrics include CPU and memory usage; cache hit ratios; response time, latency, throughput (queries per second) for APIs; build success rate and pipeline duration for CD/CI workflows.

## Useful Crates {#useful-crates .skip}

- [`metrics`][metrics.rs~website]↗ is a batteries-included instrumentation ecosystem to quickly and easily instrument your libraries and applications. It supports three fundamental metric types: counters, gauges, and histograms.
- [`tokio-metrics`][c~tokio-metrics~crates.io]↗ provides utilities for collecting metrics from a Tokio application, including runtime and per-task metrics.
- [`cadence`][c~cadence~crates.io]↗ is an extensible [`statsd`][statsd~repo]↗{{hi:statsd}} client for Rust.
- [`sentry`][c~sentry~crates.io]↗ is a ['Sentry'][getsentry~website]↗ client for Rust.

## Related Topics {#related-topics .skip}

- [[distributed_telemetry | Distributed Telemetry]].
- [[tracing | Tracing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1345)
</div>
