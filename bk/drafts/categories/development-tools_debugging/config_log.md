# Configure Logging

{{#include config_log.incl.md}}

## Enable Log Levels per Module {#enable-log-levels-per-module}

[![log][c~log~docs~badge]][c~log~docs]{{hi:log}} [![env_logger][c~env_logger~docs~badge]][c~env_logger~docs]{{hi:env_logger}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}{{hi:Log levels}}

Creates two [modules][p~modules] `foo` and nested `foo::bar` with logging directives controlled separately with [`RUST_LOG`][c~env_logger~RUST_LOG]↗{{hi:RUST_LOG}} environmental variable.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/log/log_mod.rs:example}}
```

The [`RUST_LOG`][c~env_logger~RUST_LOG]↗{{hi:RUST_LOG}} environment variable controls [`env-logger`][c~env_logger~docs]↗{{hi:env_logger}}{{hi:env_logger}} output. Module declarations take comma separated entries formatted like `path::to::module=log_level`. Run the `test` application as follows:

```bash
RUST_LOG="warn,test::foo=info,test::foo::bar=debug" test
```

Sets the default [`log::Level`][c~log::Level~docs]↗{{hi:log::Level}} to `warn`, module `foo` and module `foo::bar` to `info` and [`debug`][c~log::debug~docs]↗{{hi:debug}}.

```bash
WARN:test: [root] warn
WARN:test::foo: [foo] warn
INFO:test::foo: [foo] info
WARN:test::foo::bar: [bar] warn
INFO:test::foo::bar: [bar] info
DEBUG:test::foo::bar: [bar] debug
```

## Use a Custom Environment Variable to Set Up Logging {#custom-env-var-for-logging}

[![log][c~log~docs~badge]][c~log~docs]{{hi:log}} [![env_logger][c~env_logger~docs~badge]][c~env_logger~docs]{{hi:env_logger}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}

[`env_logger::Builder`][c~env_logger::Builder~docs]↗{{hi:env_logger::Builder}} configures logging.

[`env_logger::Builder::parse`][c~env_logger::Builder::parse~docs]↗{{hi:env_logger::Builder::parse}} parses `MY_APP_LOG`
environment variable contents in the form of [`RUST_LOG`][c~env_logger~RUST_LOG]↗{{hi:RUST_LOG}} syntax.
Then, [`env_logger::Builder::init`][c~env_logger::Builder::init~docs]↗{{hi:env_logger::Builder::init}} initializes the logger.
All these steps are normally done internally by [`env_logger::init`][c~env_logger::init~docs]↗{{hi:env_logger::init}}.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/log/log_env_variable.rs:example}}
```

## Include a Timestamp in Log Messages {#timestamp-in-log-messages}

[![log][c~log~docs~badge]][c~log~docs]{{hi:log}} [![env_logger][c~env_logger~docs~badge]][c~env_logger~docs]{{hi:env_logger}} [![chrono][c~chrono~docs~badge]][c~chrono~docs]{{hi:chrono}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}{{hi:Timestamp}}

Creates a custom logger{{hi:Custom logger}} configuration with [`env_logger::Builder`][c~env_logger::Builder~docs]↗{{hi:env_logger::Builder}}.
Each [log][p~log] entry calls [`chrono::offset::Local::now`][c~chrono::offset::Local::now~docs]↗{{hi:chrono::offset::Local::now}} to get the current [`chrono::DateTime`][c~chrono::DateTime~docs]↗{{hi:chrono::DateTime}} in local timezone and uses [`chrono::DateTime::format`][c~chrono::DateTime::format~docs]↗{{hi:chrono::DateTime::format}} with [`chrono::format::strftime`][c~chrono::format::strftime~docs]↗{{hi:chrono::format::strftime}} to format a timestamp used in the final log.

The example calls [`env_logger::Builder::format`][c~env_logger::Builder::format~docs]↗{{hi:env_logger::Builder::format}} to set a closure which formats each message text with timestamp, [`log::Record::level`][c~log::Record::level~docs]↗{{hi:log::Record::level}} and body ([`log::Record::args`][c~log::Record::args~docs]↗{{hi:log::Record::args}}).

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/log/log_timestamp.rs:example}}
```

stderr output will contain

```sh
2017-05-22T21:57:06 [WARN] - warn
2017-05-22T21:57:06 [INFO] - info
```

## Log Messages to a Custom Location {#log-to-custom-location}

[![log][c~log~docs~badge]][c~log~docs]{{hi:log}} [![log4rs][c~log4rs~docs~badge]][c~log4rs~docs]{{hi:log4rs}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}

[`log4rs`][c~log4rs~docs]↗{{hi:log4rs}} configures log output to a custom location{{hi:Log output to a custom location}}. [`log4rs`][c~log4rs~docs]↗{{hi:log4rs}} can use either an external YAML file or a builder configuration.

Create the log configuration{{hi:Log configuration}} with [`log4rs::append::file::FileAppender`][c~log4rs::append::file::FileAppender~docs]↗{{hi:log4rs::append::file::FileAppender}} An appender defines the logging destination. The configuration continues with encoding using a custom pattern from [`log4rs::encode::pattern`][c~log4rs::encode::pattern~docs]↗{{hi:log4rs::encode::pattern}} Assigns the configuration to [`log4rs::config::Config`][c~log4rs::config::Config~docs]↗{{hi:log4rs::config::Config}} and sets the default [`log::LevelFilter`][c~log::LevelFilter~docs]↗{{hi:log::LevelFilter}}.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/log/log_custom.rs:example}}
```

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/925)
</div>
