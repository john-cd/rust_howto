# Configure Logging

{{#include config_log.incl.md}}

## Enable log levels per module

[![log][c-log-badge]][c-log]  [![env_logger][c-env_logger-badge]][c-env_logger]  [![cat-debugging][cat-debugging-badge]][cat-debugging] {{hi:log levels}}

Creates two modules `foo` and nested `foo::bar` with logging directives controlled separately with [`RUST_LOG`][c-env_logger-RUST_LOG]{{hi:RUST_LOG}}⮳ environmental variable.

```rust,editable
{{#include ../../../deps/tests/log-mod.rs}}
```

The [`RUST_LOG`][c-env_logger-RUST_LOG]{{hi:RUST_LOG}} environment variable controls [`env-logger`][c-env_logger]{{hi:env-logger}}⮳ output. Module declarations take comma separated entries formatted like `path::to::module=log_level`. Run the `test` application as follows:

```bash
RUST_LOG="warn,test::foo=info,test::foo::bar=debug" test
```

Sets the default [`log::Level`][c-log::Level]{{hi:log::Level}}⮳ to `warn`, module `foo` and module `foo::bar` to `info` and `debug`.

```bash
WARN:test: [root] warn
WARN:test::foo: [foo] warn
INFO:test::foo: [foo] info
WARN:test::foo::bar: [bar] warn
INFO:test::foo::bar: [bar] info
DEBUG:test::foo::bar: [bar] debug
```

## Use a custom environment variable to set up logging

[![log][c-log-badge]][c-log]  [![env_logger][c-env_logger-badge]][c-env_logger]  [![cat-debugging][cat-debugging-badge]][cat-debugging]

[`env_logger::Builder`][c-env_logger::Builder]{{hi:env_logger::Builder}}⮳ configures logging.

[`env_logger::Builder::parse`][c-env_logger::Builder::parse]{{hi:env_logger::Builder::parse}}⮳ parses `MY_APP_LOG`
environment variable contents in the form of [`RUST_LOG`][c-env_logger-RUST_LOG]{{hi:RUST_LOG}}⮳ syntax.
Then, [`env_logger::Builder::init`][c-env_logger::Builder::init]{{hi:env_logger::Builder::init}}⮳ initializes the logger.
All these steps are normally done internally by [`env_logger::init`][c-env_logger::init]{{hi:env_logger::init}}⮳.

```rust,editable
{{#include ../../../deps/tests/log-env-variable.rs}}
```

## Include timestamp in log messages

[![log][c-log-badge]][c-log]  [![env_logger][c-env_logger-badge]][c-env_logger]  [![chrono][c-chrono-badge]][c-chrono]  [![cat-debugging][cat-debugging-badge]][cat-debugging] {{hi:Timestamp}}

Creates a custom logger{{hi:custom logger}} configuration with [`env_logger::Builder`][c-env_logger::Builder]{{hi:env_logger::Builder}}⮳
Each log entry calls [`chrono::offset::Local::now`][c-chrono::offset::Local::now]{{hi:chrono::offset::Local::now}}⮳ to get the current [`chrono::DateTime`][c-chrono::DateTime]{{hi:chrono::DateTime}}⮳ in local timezone and uses [`chrono::DateTime::format`][c-chrono::DateTime::format]{{hi:chrono::DateTime::format}}⮳ with [`chrono::format::strftime`][c-chrono::format::strftime]{{hi:chrono::format::strftime}}⮳ to format a timestamp used in the final log.

The example calls [`env_logger::Builder::format`][c-env_logger::Builder::format]{{hi:env_logger::Builder::format}}⮳ to set a closure which formats each message text with timestamp, [`log::Record::level`][c-log::Record::level]{{hi:log::Record::level}}⮳ and body ([`log::Record::args`][c-log::Record::args]{{hi:log::Record::args}}⮳).

```rust,editable
{{#include ../../../deps/tests/log-timestamp.rs}}
```

stderr output will contain

```sh
2017-05-22T21:57:06 [WARN] - warn
2017-05-22T21:57:06 [INFO] - info
```

## Log messages to a custom location

[![log][c-log-badge]][c-log]  [![log4rs][c-log4rs-badge]][c-log4rs]  [![cat-debugging][cat-debugging-badge]][cat-debugging]

[`log4rs`][c-log4rs]{{hi:log4rs}}⮳ configures log output to a custom location{{hi:log output to a custom location}}. [`log4rs`][c-log4rs]{{hi:log4rs}}⮳ can use either an external YAML file or a builder configuration.

Create the log configuration{{hi:log configuration}} with [`log4rs::append::file::FileAppender`][c-log4rs::append::file::FileAppender]{{hi:log4rs::append::file::FileAppender}}⮳ An appender defines the logging destination. The configuration continues with encoding using a custom pattern from [`log4rs::encode::pattern`][c-log4rs::encode::pattern]{{hi:log4rs::encode::pattern}}⮳ Assigns the configuration to [`log4rs::config::Config`][c-log4rs::config::Config]{{hi:log4rs::config::Config}}⮳ and sets the default [`log::LevelFilter`][c-log::LevelFilter]{{hi:log::LevelFilter}}⮳

```rust,editable,no_run
{{#include ../../../deps/tests/log-custom.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
