# Configure Logging

## Enable log levels per module

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

Creates two modules `foo` and nested `foo::bar` with logging directives
controlled separately with [`RUST_LOG`] environmental variable.

```rust,editable
{#include ../../deps/examples/log-mod.rs}
```

[`RUST_LOG`] environment variable controls [`env_logger`][env_logger] output.
Module declarations take comma separated entries formatted like
`path::to::module=log_level`. Run the `test` application as follows:

```bash
RUST_LOG="warn,test::foo=info,test::foo::bar=debug" test
```

Sets the default [`log::Level`] to `warn`, module `foo` and module `foo::bar`
to `info` and `debug`.

```bash
WARN:test: [root] warn
WARN:test::foo: [foo] warn
INFO:test::foo: [foo] info
WARN:test::foo::bar: [bar] warn
INFO:test::foo::bar: [bar] info
DEBUG:test::foo::bar: [bar] debug
```

## Use a custom environment variable to set up logging

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

[`Builder`] configures logging.

[`Builder::parse`] parses `MY_APP_LOG`
environment variable contents in the form of [`RUST_LOG`] syntax.
Then, [`Builder::init`] initializes the logger.
All these steps are normally done internally by [`env_logger::init`].

```rust,editable
{#include ../../deps/examples/log-env-variable.rs}
```

## Include timestamp in log messages

[![log-badge]][log] [![env_logger-badge]][env_logger] [![chrono-badge]][chrono] [![cat-debugging-badge]][cat-debugging]

Creates a custom logger configuration with [`Builder`].
Each log entry calls [`Local::now`] to get the current [`DateTime`] in local
timezone and uses [`DateTime::format`] with [`strftime::specifiers`] to format
a timestamp used in the final log.

The example calls [`Builder::format`] to set a closure which formats each
message text with timestamp, [`Record::level`] and body ([`Record::args`]).

```rust,editable
{#include ../../deps/examples/log-timestamp.rs}
```

stderr output will contain

```rust
{#include ../../deps/examples/log-timestamp2.rs}
```

## Log messages to a custom location

[![log-badge]][log] [![log4rs-badge]][log4rs] [![cat-debugging-badge]][cat-debugging]

[log4rs] configures log output to a custom location. [log4rs] can use either an
external YAML file or a builder configuration.

Create the log configuration with [`log4rs::append::file::FileAppender`]. An
appender defines the logging destination.  The configuration continues with
encoding using a custom pattern from [`log4rs::encode::pattern`].
Assigns the configuration to [`log4rs::config::Config`] and sets the default
[`log::LevelFilter`].

```rust,editable,no_run
{#include ../../deps/examples/log-custom.rs}
```

[`env_logger::init`]: https://docs.rs/env_logger/*/env_logger/fn.init.html
[`Builder`]: https://docs.rs/env_logger/*/env_logger/struct.Builder.html
[`Builder::init`]: https://docs.rs/env_logger/*/env_logger/struct.Builder.html#method.init
[`Builder::parse`]: https://docs.rs/env_logger/*/env_logger/struct.Builder.html#method.parse
[`RUST_LOG`]: https://docs.rs/env_logger/*/env_logger/#enabling-logging
[`DateTime::format`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.format
[`DateTime`]: https://docs.rs/chrono/*/chrono/datetime/struct.DateTime.html
[`Local::now`]: https://docs.rs/chrono/*/chrono/offset/struct.Local.html#method.now
[`Builder::format`]: https://docs.rs/env_logger/*/env_logger/struct.Builder.html#method.format
[`Record::args`]: https://docs.rs/log/*/log/struct.Record.html#method.args
[`Record::level`]: https://docs.rs/log/*/log/struct.Record.html#method.level
[`strftime::specifiers`]: https://docs.rs/chrono/*/chrono/format/strftime/index.html#specifiers
[`log::Level`]: https://docs.rs/log/*/log/enum.Level.html
[`log4rs::append::file::FileAppender`]: https://docs.rs/log4rs/*/log4rs/append/file/struct.FileAppender.html
[`log4rs::config::Config`]: https://docs.rs/log4rs/*/log4rs/config/struct.Config.html
[`log4rs::encode::pattern`]: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
[`log::LevelFilter`]: https://docs.rs/log/*/log/enum.LevelFilter.html
{{#include ../refs/link-refs.md}}
