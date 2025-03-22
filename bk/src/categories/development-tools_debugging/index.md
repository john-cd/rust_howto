# Debugging, Logging

[![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Development tools::Debugging}}

Help you figure out what is going on with your code such as logging{{hi:Logging}}, tracing{{hi:Tracing}}, or assertions{{hi:Assertions}}.

| Topic | Rust Crates |
|---|---|
| Debugging | [`gdb`][c-gdb]⮳{{hi:gdb}} or [`lldb`][c-lldb]⮳{{hi:lldb}} |
| Logging Framework | [`log`][c-log]⮳{{hi:log}} provides the logging macros and facade. [`tracing`][c-tracing]⮳{{hi:tracing}}: A newer, structured logging library with spans and context. |
| Log Implementations (Loggers) | [`env_logger`][c-env_logger]⮳{{hi:env_logger}}: A simple logger that configures logging based on environment variables. [`log4rs`][c-log4rs]⮳{{hi:log4rs}} is a more flexible logger that supports configuration files. |
| Log Formatting/Output | [`tracing-subscriber`][c-tracing_subscriber]⮳{{hi:tracing-subscriber}} is used with [`tracing`][c-tracing]⮳{{hi:tracing}} to format and output logs. |
| Asynchronous Logging | Often achieved using a logger like [`tracing`][c-tracing]⮳{{hi:tracing}} combined with asynchronous tasks. |
| Log Filtering | Can be done with [`env_logger`][c-env_logger]⮳{{hi:env_logger}}, [`log4rs`][c-log4rs]⮳{{hi:log4rs}}, or [`tracing-subscriber`][c-tracing_subscriber]⮳{{hi:tracing-subscriber}}. |
| Panic Logging | [`log-panics`][c-log_panics]⮳{{hi:log-panics}} logs panics with backtraces. |

## Tracing

{{#include tracing.incl.md}}

## Logging

{{#include log.incl.md}}

## Log Configuration

{{#include config_log.incl.md}}

## Alternatives

{{#include tracing_alternatives.incl.md}}

## Diagnostic Functions

{{#include diagnostic_functions.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[index: reorganize; dedupe alternatives / `log` / config_log](https://github.com/john-cd/rust_howto/issues/319)
</div>
