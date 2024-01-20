# Log Messages

## Log a debug message to the console

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

The `log` crate provides logging utilities. The `env_logger` crate configures
logging via an environment variable.  The [`log::debug!`] macro works like other
[`std::fmt`] formatted strings.

```rust,editable
{#include ../../../deps/examples/log-debug.rs}
```

No output prints when running this code. By default, the
log level is `error`, and any lower levels are dropped.

Set the `RUST_LOG` environment variable to print the message:

```bash
RUST_LOG=debug cargo run
```

Cargo prints debugging information then the
following line at the very end of the output:

```bash
DEBUG:main: Executing query: DROP TABLE students
```

## Log an error message to the console

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

Proper error handling considers exceptions exceptional.  Here, an error logs
to stderr with `log`'s convenience macro [`log::error!`].

```rust,editable
{#include ../../../deps/examples/log-error.rs}
```

## Log to stdout instead of stderr

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

Creates a custom logger configuration using the [`Builder::target`] to set the target of the log output to [`Target::Stdout`].

```rust,editable
{#include ../../../deps/examples/log-stdout.rs}
```

## Log messages with a custom logger

[![log-badge]][log] [![cat-debugging-badge]][cat-debugging]

Implements a custom logger `ConsoleLogger` which prints to stdout.
In order to use the logging macros, `ConsoleLogger` implements
the [`log::Log`] trait and [`log::set_logger`] installs it.

```rust,editable
{#include ../../../deps/examples/log-custom-logger.rs}
```

## Log to the Unix syslog

[![log-badge]][log] [![syslog-badge]][syslog] [![cat-debugging-badge]][cat-debugging]

Logs messages to [UNIX syslog]. Initializes logger backend
with [`syslog::init`]. [`syslog::Facility`] records the program submitting
the log entry's classification, [`log::LevelFilter`] denotes allowed log verbosity
and `Option<&str>` holds optional application name.

```rust,editable
{#include ../../../deps/examples/log-syslog.rs}
```

[`log::error!`]: https://docs.rs/log/*/log/macro.error.html
[`Builder::target`]: https://docs.rs/env_logger/*/env_logger/struct.Builder.html#method.target
[`Target::Stdout`]: https://docs.rs/env_logger/*/env_logger/fmt/enum.Target.html
[`log::Log`]: https://docs.rs/log/*/log/trait.Log.html
[`log::set_logger`]: https://docs.rs/log/*/log/fn.set_logger.html
[`log::debug!`]: https://docs.rs/log/*/log/macro.debug.html
[`std::fmt`]: https://doc.rust-lang.org/std/fmt/
[`log::LevelFilter`]: https://docs.rs/log/*/log/enum.LevelFilter.html
[`syslog::Facility`]: https://docs.rs/syslog/*/syslog/enum.Facility.html
[`syslog::init`]: https://docs.rs/syslog/*/syslog/fn.init.html
[UNIX syslog]: https://www.gnu.org/software/libc/manual/html_node/Overview-of-Syslog.html
{{#include ../refs/link-refs.md}}
