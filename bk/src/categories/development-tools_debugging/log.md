# Log Messages

{{#include log.incl.md}}

## Log a Debug Message to the Console {#log-to-console}

[![log][c-log-badge]][c-log]{{hi:log}} [![env_logger][c-env_logger-badge]][c-env_logger]{{hi:env_logger}} [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}{{hi:Debug message}}

The [`log`][c-log]{{hi:log}}⮳ crate provides logging utilities{{hi:Logging utilities}}. The [`env_logger`][c-env_logger]{{hi:env_logger}}⮳ crate configures logging via an environment variable. The [`log::debug`][c-log::debug]{{hi:log::debug}}⮳ macro works like other [`std::fmt`][c-std::fmt]{{hi:std::fmt}}⮳ formatted strings.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/tests/log/log_debug.rs:example}}
```

No output prints when running this code. By default, the log level{{hi:Log levels}} is [`error`][c-std::error::Error]⮳{{hi:error}}, and any lower levels are dropped.

Set the [`RUST_LOG`][c-env_logger-RUST_LOG]{{hi:RUST_LOG}}⮳ environment variable to print the message:

```bash
RUST_LOG=debug cargo run
```

Cargo prints debugging information{{hi:Debugging information}} then the following line at the very end of the output:

```bash
DEBUG:main: Executing query: DROP TABLE students
```

## Log an Error Message to the Console {#log-error-message-to-console}

[![log][c-log-badge]][c-log]{{hi:log}} [![env_logger][c-env_logger-badge]][c-env_logger]{{hi:env_logger}} [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}

Proper [error handling][p-error-handling]{{hi:Error handling}} considers exceptions exceptional. Here, an error logs to stderr with [`log`][c-log]{{hi:log}}'s convenience macro [`log::error`][c-log::error]{{hi:log::error}}⮳.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/tests/log/log_error.rs:example}}
```

## Log to `stdout` Instead of `stderr` {#log-to-stdout}

[![log][c-log-badge]][c-log]{{hi:log}} [![env_logger][c-env_logger-badge]][c-env_logger]{{hi:env_logger}} [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}

[`log`][c-log]⮳{{hi:log}}

[`env_logger`][c-env_logger]⮳{{hi:env_logger}}

Creates a custom logger [configuration][p-configuration]{{hi:Custom logger configuration}} using the [`env_logger::Builder::target`][c-env_logger::Builder::target]{{hi:env_logger::Builder::target}}⮳ to set the target of the log output to [`env_logger::fmt::Target`][c-env_logger::fmt::Target]{{hi:env_logger::fmt::Target}}⮳.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/tests/log/log_stdout.rs:example}}
```

## Log Messages with a Custom Logger {#custom-logger}

[![log][c-log-badge]][c-log]{{hi:log}} [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}

Implements a custom logger `ConsoleLogger` which prints to stdout. In order to use the logging [macros][p-macros], `ConsoleLogger` implements the [`log::Log`][c-log::Log]{{hi:log::Log}}⮳ trait and [`log::Log`][c-log::Log]{{hi:log::Log}}⮳ installs it.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/tests/log/log_custom_logger.rs:example}}
```

## Log to the Unix Syslog {#syslog}

[![log][c-log-badge]][c-log]{{hi:log}} [![syslog][c-syslog-badge]][c-syslog]{{hi:syslog}} [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}{{hi:syslog}}

[`log`][c-log]⮳{{hi:log}}

Logs messages to [UNIX `syslog`][unix-syslog-website]⮳. Initializes logger backend with [`syslog::init`][c-syslog::init]{{hi:syslog::init}}⮳ [`syslog::init`][c-syslog::init]{{hi:syslog::init}}⮳ records the program submitting the log entry's classification, [`syslog::init`][c-syslog::init]{{hi:syslog::init}}⮳ denotes allowed log verbosity{{hi:Log verbosity}} and `Option<&str>` holds optional application name.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/tests/log/log_syslog.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/927)
</div>
