# Debugging, Logging

[![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Development tools::Debugging}}

Help you figure out what is going on with your code such as logging{{hi:Logging}}, tracing{{hi:Tracing}}, or assertions{{hi:Assertions}}.

## Tracing

{{#include tracing.incl.md}}

## Logging

{{#include log.incl.md}}

## Log Configuration

{{#include config_log.incl.md}}

## Alternatives

{{#include tracing_alternatives.incl.md}}

## Diagnostic functions

{{#include diagnostic_functions.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[index: reorganize; dedupe alternatives / log / config_log (P1)](https://github.com/john-cd/rust_howto/issues/319)

Debugging with `gdb` or `lldb`: Basic debugging techniques.

Logging Framework: `log` (provides the logging macros and facade)

Log Implementations (Loggers):

`env_logger`: A simple logger that configures logging based on environment variables.
`log4rs`: A more flexible logger that supports configuration files.
`tracing`: A newer, structured logging library with spans and context.

Log Formatting/Output:

`tracing-subscriber`: Used with tracing to format and output logs.

Asynchronous Logging:

(Often achieved using a logger like tracing combined with asynchronous tasks.)

Log Filtering:

(Can be done with `env_logger`, `log4rs`, or `tracing-subscriber`.)

Panic Logging:

`log-panics`: Logs panics with backtraces.
</div>
