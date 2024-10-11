# Configure Logging

{{#include config_log.incl.md}}

## Enable log levels per module

[![log][c-log-badge]][c-log]  [![env_logger][c-env_logger-badge]][c-env_logger]  [![cat-debugging][cat-debugging-badge]][cat-debugging] {{hi:log levels}}

Creates two modules `foo` and nested `foo::bar` with logging directives controlled separately with [`{{i:RUST_LOG}}`][c-env_logger-RUST_LOG]⮳ environmental variable.

```rust,editable
{{#include ../../../deps/tests/log-mod.rs}}
```

The [`{{i:RUST_LOG}}`][c-env_logger-RUST_LOG] environment variable controls [`{{i:env-logger}}`][c-env_logger]⮳ output. Module declarations take comma separated entries formatted like `path::to::module=log_level`. Run the `test` application as follows:

```bash
RUST_LOG="warn,test::foo=info,test::foo::bar=debug" test
```

Sets the default [`{{i:log::Level}}`][c-log::Level]⮳ to `warn`, module `foo` and module `foo::bar` to `info` and `debug`.

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

[`{{i:Builder}}`][c-env_logger::Builder]⮳ configures logging.

[`{{i:Builder::parse}}`][c-env_logger::Builder::parse]⮳ parses `MY_APP_LOG`
environment variable contents in the form of [`{{i:RUST_LOG}}`][c-env_logger-RUST_LOG]⮳ syntax.
Then, [`{{i:Builder::init}}`][c-env_logger::Builder::init]⮳ initializes the logger.
All these steps are normally done internally by [`{{i:env_logger::init}}`][c-env_logger::init]⮳.

```rust,editable
{{#include ../../../deps/tests/log-env-variable.rs}}
```

## Include timestamp in log messages

[![log][c-log-badge]][c-log]  [![env_logger][c-env_logger-badge]][c-env_logger]  [![chrono][c-chrono-badge]][c-chrono]  [![cat-debugging][cat-debugging-badge]][cat-debugging] {{hi:timestamp}}

Creates a {{i:custom logger}} configuration with [`{{i:Builder}}`][c-env_logger::Builder]⮳
Each log entry calls [`{{i:Local::now}}`][c-chrono::offset::Local::now]⮳ to get the current [`{{i:DateTime}}`][c-chrono::DateTime]⮳ in local timezone and uses [`{{i:DateTime::format}}`][c-chrono::DateTime::format]⮳ with [`{{i:strftime::specifiers}}`][c-chrono::format::strftime]⮳ to format a timestamp used in the final log.

The example calls [`{{i:Builder::format}}`][c-env_logger::Builder::format]⮳ to set a closure which formats each message text with timestamp, [`{{i:Record::level}}`][c-log::Record::level]⮳ and body ([`{{i:Record::args}}`][c-log::Record::args]⮳).

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

[`{{i:log4rs}}`][c-log4rs]⮳ configures {{i:log output to a custom location}}. [`{{i:log4rs}}`][c-log4rs]⮳ can use either an external YAML file or a builder configuration.

Create the {{i:log configuration}} with [`{{i:log4rs::append::file::FileAppender}}`][c-log4rs::append::file::FileAppender]⮳ An appender defines the logging destination. The configuration continues with encoding using a custom pattern from [`{{i:log4rs::encode::pattern}}`][c-log4rs::encode::pattern]⮳ Assigns the configuration to [`{{i:log4rs::config::Config}}`][c-log4rs::config::Config]⮳ and sets the default [`{{i:log::LevelFilter}}`][c-log::LevelFilter]⮳

```rust,editable,no_run
{{#include ../../../deps/tests/log-custom.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
