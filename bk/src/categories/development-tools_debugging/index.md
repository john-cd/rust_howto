<!-- TODO Reviewed ??-??-2026. -->

# Development Tools - Debugging

[![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Development tools::Debugging}}

Debugging tools help explain what a program is doing and why it behaves unexpectedly. Use a debugger to pause execution and inspect state, logs to record selected events, tracing spans to follow work through asynchronous code and services, and metrics to summarize behavior over time.

## Debuggers

| Topic | Tools |
| --- | --- |
| [Debuggers](debugger.md) | `rust-gdb`, `rust-lldb`, or VS Code with CodeLLDB |

For quick value inspection and panic backtraces, see [Debugging](debugging.md).

{{#include debugging.incl.md}}

For source navigation, [Lantern Lattice][lantern-lattice~repo]↗ builds a call graph using language tooling such as rust-analyzer. Check the tool's target, feature, and generated-code coverage when a call edge matters.

## Logging and Tracing

### `tracing`

The [`tracing`][c~tracing~docs]↗ ecosystem records structured events and spans. Its subscribers collect, filter, format, and write those records; spans can retain context while asynchronous tasks are polled. See [Tracing](tracing.md) for subscriber setup, filtering, formatting, file output, and span instrumentation.

{{#include tracing.incl.md}}

### The `log` Facade and Other Implementations

Use [`log`][c~log~docs]↗ when a library needs to emit level-based records without choosing an application's logger. An executable initializes one compatible implementation. [`env_logger`][c~env_logger~docs]↗ is a small environment-configured choice; [`log4rs`][c~log4rs~docs]↗ supports configurable appenders and formats. See [Alternatives](tracing_alternatives.md) for `slog` and compatibility notes.

{{#include log.incl.md}}

### Configure Output and Filtering

Configure the selected logger once near application startup. Environment filters are useful for local runs; file configuration can set output destinations and formats for deployment.

{{#include config_log.incl.md}}

## Diagnostic Functions

Use `dbg!`, backtraces, and source-location macros for temporary or targeted inspection. Type-name functions are diagnostic hints; their spelling is not a stable identifier.

{{#include diagnostic_functions.incl.md}}

## Metrics

Metrics aggregate measurements such as request totals and latency distributions. They complement logs and traces by showing rates and trends without retaining each event.

{{#include metrics.incl.md}}

## Distributed Telemetry

OpenTelemetry exports traces to a collector or compatible backend. The examples use local exporters by default and need a configured collector only when you opt into network export.

{{#include distributed_telemetry.incl.md}}

## Related Topics {#related-topics .skip}

- [Development Tools - Testing](../development-tools_testing/index.md) for tests, fuzzing, and coverage.
- [Development Tools - Profiling](../development-tools_profiling/index.md) for CPU, memory, and latency analysis.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
