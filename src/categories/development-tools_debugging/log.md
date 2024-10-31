# Log Messages

{{#include log.incl.md}}

## Log a debug message to the console

[![log][c-log-badge]][c-log]{{hi:log}}  [![env_logger][c-env_logger-badge]][c-env_logger]{{hi:env_logger}}  [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}{{hi:Debug message}}

The [`log`][c-log]{{hi:log}}⮳ crate provides logging utilities{{hi:Logging utilities}}. The [`env_logger`][c-env_logger]{{hi:env_logger}}⮳ crate configures logging via an environment variable. The [`log::debug`][c-log::debug]{{hi:log::debug}}⮳ macro works like other
[`std::fmt`][c-std::fmt]{{hi:std::fmt}}⮳ formatted strings.

```rust
{{#include ../../../deps/tests/log_debug.rs}}
```

No output prints when running this code. By default, the log level{{hi:Log levels}} is `error`, and any lower levels are dropped.

Set the [`RUST_LOG`][c-env_logger-RUST_LOG]{{hi:RUST_LOG}}⮳ environment variable to print the message:

```bash
RUST_LOG=debug cargo run
```

Cargo prints debugging information{{hi:Debugging information}} then the following line at the very end of the output:

```bash
DEBUG:main: Executing query: DROP TABLE students
```

## Log an error message to the console

[![log][c-log-badge]][c-log]{{hi:log}}  [![env_logger][c-env_logger-badge]][c-env_logger]{{hi:env_logger}}  [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}

Proper error handling{{hi:Error handling}} considers exceptions exceptional. Here, an error logs to stderr with [`log`][c-log]{{hi:log}}'s convenience macro [`log::error`][c-log::error]{{hi:log::error}}⮳.

```rust
{{#include ../../../deps/tests/log_error.rs}}
```

## Log to stdout instead of stderr

[![log][c-log-badge]][c-log]{{hi:log}}  [![env_logger][c-env_logger-badge]][c-env_logger]{{hi:env_logger}}  [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}

Creates a custom logger configuration{{hi:Custom logger configuration}} using the [`env_logger::Builder::target`][c-env_logger::Builder::target]{{hi:env_logger::Builder::target}}⮳ to set the target of the log output to [`env_logger::fmt::Target`][c-env_logger::fmt::Target]{{hi:env_logger::fmt::Target}}⮳

```rust
{{#include ../../../deps/tests/log_stdout.rs}}
```

## Log messages with a custom logger

[![log][c-log-badge]][c-log]{{hi:log}}  [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}

Implements a custom logger `ConsoleLogger` which prints to stdout. In order to use the logging macros, `ConsoleLogger` implements the [`log::Log`][c-log::Log]{{hi:log::Log}}⮳ trait and [`log::Log`][c-log::Log]{{hi:log::Log}}⮳ installs it.

```rust
{{#include ../../../deps/tests/log_custom_logger.rs}}
```

## Log to the Unix syslog

[![log][c-log-badge]][c-log]{{hi:log}}  [![syslog][c-syslog-badge]][c-syslog]{{hi:syslog}}  [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}{{hi:syslog}}

Logs messages to [UNIX `syslog`][unix-syslog-website]⮳. Initializes logger backend with [`syslog::init`][c-syslog::init]{{hi:syslog::init}}⮳  [`syslog::init`][c-syslog::init]{{hi:syslog::init}}⮳ records the program submitting the log entry's classification, [`syslog::init`][c-syslog::init]{{hi:syslog::init}}⮳ denotes allowed log verbosity{{hi:Log verbosity}} and `Option<&str>` holds optional application name.

```rust
{{#include ../../../deps/tests/log_syslog.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO
Tracing is now the go-to crate for logging.

log
An older and simpler crate if your needs are simple and you are not using any async code.
</div>
