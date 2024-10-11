# Log Messages

{{#include log.incl.md}}

## Log a debug message to the console

[![log][c-log-badge]][c-log]  [![env_logger][c-env_logger-badge]][c-env_logger]  [![cat-debugging][cat-debugging-badge]][cat-debugging] {{hi:debug message}}

The [`{{i:log}}`][c-log]⮳ crate provides {{i:logging utilities}}. The [`{{i:env_logger}}`][c-env_logger]⮳ crate configures logging via an environment variable. The [`{{i:log::debug!}}`][c-log::debug]⮳ macro works like other
[`{{i:std::fmt}}`][c-std::fmt]⮳ formatted strings.

```rust,editable
{{#include ../../../deps/tests/log-debug.rs}}
```

No output prints when running this code. By default, the {{i:log level}} is `error`, and any lower levels are dropped.

Set the [`{{i:RUST_LOG}}`][c-env_logger-RUST_LOG]⮳ environment variable to print the message:

```bash
RUST_LOG=debug cargo run
```

Cargo prints {{i:debugging information}} then the following line at the very end of the output:

```bash
DEBUG:main: Executing query: DROP TABLE students
```

## Log an error message to the console

[![log][c-log-badge]][c-log]  [![env_logger][c-env_logger-badge]][c-env_logger]  [![cat-debugging][cat-debugging-badge]][cat-debugging]

Proper {{i:error handling}} considers exceptions exceptional. Here, an error logs to stderr with [`{{i:log}}`][c-log]'s convenience macro [`{{i:log::error!}}`][c-log::error]⮳.

```rust,editable
{{#include ../../../deps/tests/log-error.rs}}
```

## Log to stdout instead of stderr

[![log][c-log-badge]][c-log]  [![env_logger][c-env_logger-badge]][c-env_logger]  [![cat-debugging][cat-debugging-badge]][cat-debugging]

Creates a {{i:custom logger configuration}} using the [`{{i:Builder::target}}`][c-env_logger::Builder::target]⮳ to set the target of the log output to [`{{i:Target::Stdout}}`][c-env_logger::fmt::Target]⮳

```rust,editable
{{#include ../../../deps/tests/log-stdout.rs}}
```

## Log messages with a custom logger

[![log][c-log-badge]][c-log]  [![cat-debugging][cat-debugging-badge]][cat-debugging]

Implements a custom logger `ConsoleLogger` which prints to stdout. In order to use the logging macros, `ConsoleLogger` implements the [`{{i:log::Log}}`][c-log::Log]⮳ trait and [`{{i:log::set_logger}}`][c-log::Log]⮳ installs it.

```rust,editable
{{#include ../../../deps/tests/log-custom-logger.rs}}
```

## Log to the Unix syslog

[![log][c-log-badge]][c-log]  [![syslog][c-syslog-badge]][c-syslog]  [![cat-debugging][cat-debugging-badge]][cat-debugging] {{hi:syslog}}

Logs messages to [UNIX `syslog`][unix-syslog-website]⮳. Initializes logger backend with [`{{i:syslog::init}}`][c-syslog::init]⮳  [`{{i:syslog::Facility}}`][c-syslog::init]⮳ records the program submitting the log entry's classification, [`{{i:log::LevelFilter}}`][c-syslog::init]⮳ denotes allowed {{i:log verbosity}} and `Option<&str>` holds optional application name.

```rust,editable
{{#include ../../../deps/tests/log-syslog.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
