<!-- TODO Reviewed ??-??-2026 -->

# Log Messages

{{#include log.incl.md}}

## Log a Debug Message to the Console {#log-to-console}

[![log][c~log~docs~badge]][c~log~docs]{{hi:log}} [![env_logger][c~env_logger~docs~badge]][c~env_logger~docs]{{hi:env_logger}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}{{hi:Debug message}}

The [`log`][c~log~docs]↗{{hi:log}} crate provides logging utilities{{hi:Logging utilities}}. The [`env_logger`][c~env_logger~docs]↗{{hi:env_logger}} crate configures logging via an environment variable. The [`log::debug`][c~log::debug~docs]↗{{hi:log::debug}} macro works like other [`std::fmt`][c~std::fmt~docs]↗{{hi:std::fmt}} formatted strings.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/log/log_debug.rs:example}}
```

No output prints when running this code. By default, [`env_logger`][c~env_logger~docs]↗{{hi:env_logger}} filters at the [`error`][c~log::LevelFilter~docs]↗{{hi:error}} level, so lower levels are dropped.

Set the [`RUST_LOG`][c~env_logger~RUST_LOG]↗{{hi:RUST_LOG}} environment variable to print the message:

```bash
RUST_LOG=debug cargo run
```

Cargo prints debugging information{{hi:Debugging information}} then the following line at the very end of the output:

```bash
DEBUG:main: Executing query: DROP TABLE students
```

## Log an Error Message to the Console {#log-error-message-to-console}

[![log][c~log~docs~badge]][c~log~docs]{{hi:log}} [![env_logger][c~env_logger~docs~badge]][c~env_logger~docs]{{hi:env_logger}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}

Proper [error handling][p~error-handling]↗{{hi:Error handling}} considers exceptions exceptional. Here, an error logs to stderr with [`log`][c~log~docs]{{hi:log}}'s convenience macro [`log::error`][c~log::error~docs]↗{{hi:log::error}}.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/log/log_error.rs:example}}
```

## Log to `stdout` Instead of `stderr` {#log-to-stdout}

[![log][c~log~docs~badge]][c~log~docs]{{hi:log}} [![env_logger][c~env_logger~docs~badge]][c~env_logger~docs]{{hi:env_logger}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}

This example creates a custom logger [configuration][p~configuration]↗{{hi:Custom logger configuration}} with [`env_logger::Builder::target`][c~env_logger::Builder::target~docs]↗{{hi:env_logger::Builder::target}} to send log output to [`env_logger::fmt::Target`][c~env_logger::fmt::Target~docs]↗{{hi:env_logger::fmt::Target}}.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/log/log_stdout.rs:example}}
```

## Log Messages with a Custom Logger {#custom-logger}

[![log][c~log~docs~badge]][c~log~docs]{{hi:log}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}

The custom `ConsoleLogger` prints records to stdout. It implements the [`log::Log`][c~log::Log~docs]↗{{hi:log::Log}} trait; [`log::set_logger`][c~log::set_logger~docs]↗{{hi:log::set_logger}} installs it for the logging [macros][p~macros].

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/log/log_custom_logger.rs:example}}
```

## Log to Unix Syslog {#syslog}

[![log][c~log~docs~badge]][c~log~docs]{{hi:log}} [![syslog][c~syslog~docs~badge]][c~syslog~docs]{{hi:syslog}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}{{hi:syslog}}

This example uses [`syslog::init`][c~syslog::init~docs]↗{{hi:syslog::init}} to install a Unix syslog backend, setting its facility, maximum log level, and optional application name.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/log/log_syslog.rs:example}}
```

`log` is a lightweight facade for unstructured records. For structured events, spans, and asynchronous context, see [`tracing`][p~tracing]↗{{hi:tracing}}.

## Related Topics {#related-topics .skip}

- [Configure logging][ex~development-tools_debugging~custom-env-var-for-logging]↗.
- [Tracing basics][ex~development-tools_debugging~tracing-basics]↗.
- [Diagnostic functions][ex~development-tools_debugging~type-name-of-val]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
