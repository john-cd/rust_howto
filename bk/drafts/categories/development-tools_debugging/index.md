# Debugging, Logging

[![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Development tools::Debugging}}

Help you figure out what is going on with your code such as logging{{hi:Logging}}, tracing{{hi:Tracing}}, or assertions{{hi:Assertions}}.

## Debuggers

| Topic | Rust Crates |
|---|---|
| Debugging | [`gdb`][c~gdb~docs]⮳{{hi:gdb}} or [`lldb`][c~lldb~docs]⮳{{hi:lldb}} |

{{#include debugging.incl.md}}

## Logging & Tracing

### `tracing`

[`tracing`][c~tracing~docs]⮳{{hi:tracing}} is a newer, more structured logging library with support for spans and context. Supports asynchronous logging. Often used with [`tracing-subscriber`][c~tracing_subscriber~docs]⮳{{hi:tracing-subscriber}} for [formatting][p~formatting] and output.

[`tracing-subscriber`][c~tracing_subscriber~docs]⮳{{hi:tracing-subscriber}} is used with [tracing][p~tracing] to format log output, often to JSON or other structured formats. Can also be used for filtering.

{{#include tracing.incl.md}}

### Older Frameworks

{{#include tracing_alternatives.incl.md}}

### `log` and Friends

| Topic | Rust Crates |
|---|---|
| Logging Framework | [`log`][c~log~docs]⮳{{hi:log}} is an older crate providing logging macros and facade. |
| Log Implementations (Loggers) | [`env_logger`][c~env_logger~docs]⮳{{hi:env_logger}}: A simple logger that configures logging based on environment variables. [`log4rs`][c~log4rs~docs]⮳{{hi:log4rs}} is a more flexible logger that supports  [configuration][p~configuration] files (YAML, TOML, JSON). |
| Log Filtering | Can be done with [`env_logger`][c~env_logger~docs]⮳{{hi:env_logger}}, [`log4rs`][c~log4rs~docs]⮳{{hi:log4rs}}. |

{{#include log.incl.md}}

{{#include config_log.incl.md}}

## Panics & Logging

[`log-panics`][c~log_panics~docs]⮳{{hi:log-panics}} logs panics with backtraces.

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

<div class="hidden">
[index: reorganize; dedupe alternatives / `log` / config_log](https://github.com/john-cd/rust_howto/issues/319)
</div>
