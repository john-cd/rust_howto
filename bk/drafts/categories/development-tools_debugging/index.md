<!-- TODO Reviewed ??-??-2026 -->

# Debugging, Logging

[![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Development tools::Debugging}}

Helps figure out what is going on with your code such as logging{{hi:Logging}}, tracing{{hi:Tracing}}, or assertions{{hi:Assertions}}.

## Debuggers

| Topic      | Rust Crates |
| ---------- | ----------- |
| [[debugger | Debuggers]] | `rust-gdb`, `rust-lldb`, or VS Code with CodeLLDB |

For quick value inspection and panic backtraces, see [[debugging | Debugging]].

{{#include debugging.incl.md}}

## Logging & Tracing

### `tracing`

[`tracing`][c~tracing~docs]↗{{hi:tracing}} is a newer, more structured logging library with support for spans and context. Supports asynchronous logging. Often used with [`tracing-subscriber`][c~tracing-subscriber~docs]↗{{hi:tracing-subscriber}} for [formatting][p~formatting] and output.

[`tracing-subscriber`][c~tracing-subscriber~docs]↗{{hi:tracing-subscriber}} is used with [tracing][p~tracing] to format log output, often to JSON or other structured formats. Can also be used for filtering.

{{#include tracing.incl.md}}

### The `log` Facade and Other Implementations

Use [`log`][c~log~docs]↗{{hi:log}} when a library needs to emit records without choosing an application's logger. An executable initializes one compatible implementation; `env_logger` is a small environment-configured choice, while `log4rs` supports file-based configuration. The recipes below are the single index for these alternatives.

{{#include log.incl.md}}

### Configure Output and Filtering

Configure the selected logger once near application startup. Environment filters are convenient for local runs; file configuration is useful when output destinations or formats belong to deployment settings. The examples below cover these choices.

{{#include config_log.incl.md}}

### Other Logging Frameworks

[`slog`][c~slog~docs]↗{{hi:slog}} is another logging facade with structured key-value records. The alternatives table keeps this separate from the `log` facade and its `env_logger` and `log4rs` implementations.

{{#include tracing_alternatives.incl.md}}

## Panics & Logging

[`log-panics`][c~log-panics~docs]↗{{hi:log-panics}} logs panics with backtraces.

## Diagnostic Functions

{{#include diagnostic_functions.incl.md}}

## Metrics

{{#include metrics.incl.md}}

## Distributed Telemetry

{{#include distributed_telemetry.incl.md}}

## Related Topics

- [[development-tools | Development Tools]].
- [[development-tools_profiling | Development Tools: Profiling]].
- [[development-tools_testing | Development Tools: Testing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
