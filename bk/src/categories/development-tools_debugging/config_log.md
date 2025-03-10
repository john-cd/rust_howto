# Configure Logging

{{#include config_log.incl.md}}

## Enable log levels per module {#enable-log-levels-per-module}

[![log][c-log-badge]][c-log]{{hi:log}} [![env_logger][c-env_logger-badge]][c-env_logger]{{hi:env_logger}} [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}{{hi:Log levels}}

Creates two [modules][p-modules] `foo` and nested `foo::bar` with logging directives controlled separately with [`RUST_LOG`][c-env_logger-RUST_LOG]{{hi:RUST_LOG}}⮳ environmental variable.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/tests/log/log_mod.rs:example}}
```

The [`RUST_LOG`][c-env_logger-RUST_LOG]{{hi:RUST_LOG}} environment variable controls [`env-logger`][c-env_logger]{{hi:env_logger}}{{hi:env_logger}}⮳ output. Module declarations take comma separated entries formatted like `path::to::module=log_level`. Run the `test` application as follows:

```bash
RUST_LOG="warn,test::foo=info,test::foo::bar=debug" test
```

Sets the default [`log::Level`][c-log::Level]{{hi:log::Level}}⮳ to `warn`, module `foo` and module `foo::bar` to `info` and [`debug`][c-log::debug]⮳{{hi:debug}}.

```bash
WARN:test: [root] warn
WARN:test::foo: [foo] warn
INFO:test::foo: [foo] info
WARN:test::foo::bar: [bar] warn
INFO:test::foo::bar: [bar] info
DEBUG:test::foo::bar: [bar] debug
```

## Use a custom environment variable to set up logging {#custom-env-var-for-logging}

[![log][c-log-badge]][c-log]{{hi:log}} [![env_logger][c-env_logger-badge]][c-env_logger]{{hi:env_logger}} [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}

[`env_logger::Builder`][c-env_logger::Builder]{{hi:env_logger::Builder}}⮳ configures logging.

[`env_logger::Builder::parse`][c-env_logger::Builder::parse]{{hi:env_logger::Builder::parse}}⮳ parses `MY_APP_LOG`
environment variable contents in the form of [`RUST_LOG`][c-env_logger-RUST_LOG]{{hi:RUST_LOG}}⮳ syntax.
Then, [`env_logger::Builder::init`][c-env_logger::Builder::init]{{hi:env_logger::Builder::init}}⮳ initializes the logger.
All these steps are normally done internally by [`env_logger::init`][c-env_logger::init]{{hi:env_logger::init}}⮳.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/tests/log/log_env_variable.rs:example}}
```

## Include a timestamp in log messages {#timestamp-in-log-messages}

[![log][c-log-badge]][c-log]{{hi:log}} [![env_logger][c-env_logger-badge]][c-env_logger]{{hi:env_logger}} [![chrono][c-chrono-badge]][c-chrono]{{hi:chrono}} [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}{{hi:Timestamp}}

Creates a custom logger{{hi:Custom logger}} configuration with [`env_logger::Builder`][c-env_logger::Builder]{{hi:env_logger::Builder}}⮳.
Each [log][p-log] entry calls [`chrono::offset::Local::now`][c-chrono::offset::Local::now]{{hi:chrono::offset::Local::now}}⮳ to get the current [`chrono::DateTime`][c-chrono::DateTime]{{hi:chrono::DateTime}}⮳ in local timezone and uses [`chrono::DateTime::format`][c-chrono::DateTime::format]{{hi:chrono::DateTime::format}}⮳ with [`chrono::format::strftime`][c-chrono::format::strftime]{{hi:chrono::format::strftime}}⮳ to format a timestamp used in the final log.

The example calls [`env_logger::Builder::format`][c-env_logger::Builder::format]{{hi:env_logger::Builder::format}}⮳ to set a closure which formats each message text with timestamp, [`log::Record::level`][c-log::Record::level]{{hi:log::Record::level}}⮳ and body ([`log::Record::args`][c-log::Record::args]{{hi:log::Record::args}}⮳).

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/tests/log/log_timestamp.rs:example}}
```

stderr output will contain

```sh
2017-05-22T21:57:06 [WARN] - warn
2017-05-22T21:57:06 [INFO] - info
```

## Log messages to a custom location {#log-to-custom-location}

[![log][c-log-badge]][c-log]{{hi:log}} [![log4rs][c-log4rs-badge]][c-log4rs]{{hi:log4rs}} [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}

[`log4rs`][c-log4rs]{{hi:log4rs}}⮳ configures log output to a custom location{{hi:Log output to a custom location}}. [`log4rs`][c-log4rs]{{hi:log4rs}}⮳ can use either an external YAML file or a builder configuration.

Create the log configuration{{hi:Log configuration}} with [`log4rs::append::file::FileAppender`][c-log4rs::append::file::FileAppender]{{hi:log4rs::append::file::FileAppender}}⮳ An appender defines the logging destination. The configuration continues with encoding using a custom pattern from [`log4rs::encode::pattern`][c-log4rs::encode::pattern]{{hi:log4rs::encode::pattern}}⮳ Assigns the configuration to [`log4rs::config::Config`][c-log4rs::config::Config]{{hi:log4rs::config::Config}}⮳ and sets the default [`log::LevelFilter`][c-log::LevelFilter]{{hi:log::LevelFilter}}⮳.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/tests/log/log_custom.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/925)

Logging Framework: [log][p-log] (provides the logging [macros][p-macros] and facade)

Log Implementations (Loggers):

[`env_logger`][c-env_logger]⮳{{hi:env_logger}}: A popular logger that configures logging based on environment variables.
[`log4rs`][c-log4rs]⮳{{hi:log4rs}}: A more flexible logger that supports [configuration][p-configuration] files (YAML, TOML, JSON).
[`tracing`][c-tracing]⮳{{hi:tracing}}: A newer, more structured logging library with support for spans and context. Often used with [`tracing-subscriber`][c-tracing_subscriber]⮳{{hi:tracing-subscriber}} for [formatting][p-formatting] and output.

Log [Formatting][p-formatting]:

[`tracing-subscriber`][c-tracing_subscriber]⮳{{hi:tracing-subscriber}}: Used with [tracing][p-tracing] to format log output, often to JSON or other structured formats. Can also be used for filtering.
Configuration Files (for loggers like [log][p-log]4rs):

[`serde`][c-serde]⮳{{hi:serde}}: (Not a logging crate itself, but necessary for deserializing [configuration][p-configuration] files in formats like YAML, TOML, or JSON)
yaml-rust, serde_yml, toml, serde_json: (Crates for [parsing][p-parsing] the respective [configuration][p-configuration] file formats)
Other Utilities:

[`log-panics`][c-log_panics]⮳{{hi:log-panics}}: Logs panics with backtraces.
</div>
