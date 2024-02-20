# Log Messages

## Log a debug message to the console

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

The `log` crate provides logging utilities. The `env_logger` crate configures logging via an environment variable. The [`log::debug!`][log::debug] macro works like other
[`std::fmt`][std::fmt] formatted strings.

```rust,editable
{{#include ../../deps/tests/log-debug.rs}}
```

No output prints when running this code. By default, the log level is `error`, and any lower levels are dropped.

Set the `RUST_LOG` environment variable to print the message:

```bash
RUST_LOG=debug cargo run
```

Cargo prints debugging information then the following line at the very end of the output:

```bash
DEBUG:main: Executing query: DROP TABLE students
```

## Log an error message to the console

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

Proper error handling considers exceptions exceptional. Here, an error logs to stderr with `log`'s convenience macro [`log::error!`][log::error]

```rust,editable
{{#include ../../deps/tests/log-error.rs}}
```

## Log to stdout instead of stderr

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

Creates a custom logger configuration using the [`Builder::target`][env_logger::Builder::target] to set the target of the log output to [`Target::Stdout`][env_logger::fmt::Target]

```rust,editable
{{#include ../../deps/tests/log-stdout.rs}}
```

## Log messages with a custom logger

[![log-badge]][log] [![cat-debugging-badge]][cat-debugging]

Implements a custom logger `ConsoleLogger` which prints to stdout. In order to use the logging macros, `ConsoleLogger` implements the [`log::Log`][log::Log] trait and [`log::set_logger`][log::set_logger] installs it.

```rust,editable
{{#include ../../deps/tests/log-custom-logger.rs}}
```

## Log to the Unix syslog

[![log-badge]][log] [![syslog-badge]][syslog] [![cat-debugging-badge]][cat-debugging]

Logs messages to [UNIX `syslog`][UNIX syslog]. Initializes logger backend with [`syslog::init`][syslog::init] [`syslog::Facility`][syslog::Facility] records the program submitting the log entry's classification, [`log::LevelFilter`][log::LevelFilter] denotes allowed log verbosity and `Option<&str>` holds optional application name.

```rust,editable
{{#include ../../deps/tests/log-syslog.rs}}
```

{{#include ../refs/link-refs.md}}
