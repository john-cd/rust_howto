# Configure Logging

## Enable log levels per module

[![log-badge]][log]  [![env_logger-badge]][env_logger]  [![cat-debugging-badge]][cat-debugging]

Creates two modules `foo` and nested `foo::bar` with logging directives controlled separately with [`RUST_LOG`][env_logger-RUST_LOG] environmental variable.

```rust,editable
{{#include ../../deps/examples/log-mod.rs}}
```

The `RUST_LOG` environment variable controls [`env-logger`][env_logger] output. Module declarations take comma separated entries formatted like
`path::to::module=log_level`. Run the `test` application as follows:

```bash
RUST_LOG="warn,test::foo=info,test::foo::bar=debug" test
```

Sets the default [`log::Level`][log::Level] to `warn`, module `foo` and module `foo::bar` to `info` and `debug`.

```bash
WARN:test: [root] warn
WARN:test::foo: [foo] warn
INFO:test::foo: [foo] info
WARN:test::foo::bar: [bar] warn
INFO:test::foo::bar: [bar] info
DEBUG:test::foo::bar: [bar] debug
```

## Use a custom environment variable to set up logging

[![log-badge]][log]  [![env_logger-badge]][env_logger]  [![cat-debugging-badge]][cat-debugging]

[`Builder`][env_logger::Builder] configures logging.

[`Builder::parse`][env_logger::Builder::parse] parses `MY_APP_LOG`
environment variable contents in the form of [`RUST_LOG`][env_logger-RUST_LOG] syntax.
Then, [`Builder::init`][env_logger::Builder::init] initializes the logger.
All these steps are normally done internally by [`env_logger::init`][env_logger::init].

```rust,editable
{{#include ../../deps/examples/log-env-variable.rs}}
```

## Include timestamp in log messages

[![log-badge]][log]  [![env_logger-badge]][env_logger]  [![chrono-badge]][chrono]  [![cat-debugging-badge]][cat-debugging]

Creates a custom logger configuration with [`Builder`][env_logger::Builder]
Each log entry calls [`Local::now`][chrono::offset::Local::now] to get the current [`DateTime`][chrono::DateTime] in local timezone and uses [`DateTime::format`][chrono::DateTime::format] with [`strftime::specifiers`][chrono::format::strftime] to format a timestamp used in the final log.

The example calls [`Builder::format`][env_logger::Builder::format] to set a closure which formats each message text with timestamp, [`Record::level`][log::Record::level] and body ([`Record::args`][log::Record::args]).

```rust,editable
{{#include ../../deps/examples/log-timestamp.rs}}
```

stderr output will contain

```sh
2017-05-22T21:57:06 [WARN] - warn
2017-05-22T21:57:06 [INFO] - info
```

## Log messages to a custom location

[![log-badge]][log] [![log4rs-badge]][log4rs] [![cat-debugging-badge]][cat-debugging]

[`log4rs`][log4rs] configures log output to a custom location. [`log4rs`][log4rs] can use either an external YAML file or a builder configuration.

Create the log configuration with [`log4rs::append::file::FileAppender`][log4rs::append::file::FileAppender] An appender defines the logging destination. The configuration continues with
encoding using a custom pattern from [`log4rs::encode::pattern`][log4rs::encode::pattern] Assigns the configuration to [`log4rs::config::Config`][log4rs::config::Config] and sets the default
[`log::LevelFilter`][log::LevelFilter]

```rust,editable,no_run
{{#include ../../deps/examples/log-custom.rs}}
```

{{#include ../refs/link-refs.md}}
