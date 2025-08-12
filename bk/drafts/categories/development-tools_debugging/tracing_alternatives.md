# Alternatives

{{#include tracing_alternatives.incl.md}}

## Use Older Alternatives to `tracing` {#skip1}

[`tracing`][c~tracing~docs]↗{{hi:tracing}} is now the "go-to" crate for logging, but [`log`][c~log~docs]↗{{hi:log}}, [`slog`][c~slog~docs]↗{{hi:slog}} and [`log4rs`][c~log4rs~docs]↗{{hi:log4rs}} are still in extensive use.

### `log` {#log}

[![log][c~log~docs~badge]][c~log~docs] [![log~crates.io][c~log~crates.io~badge]][c~log~crates.io] [![log~github][c~log~github~badge]][c~log~github] [![log~lib.rs][c~log~lib.rs~badge]][c~log~lib.rs]{{hi:log}}{{hi:Logging}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}

[`log`][c~log~docs]↗{{hi:log}} is an older and simpler crate, if your needs are simple and you are not using any [async][p~async] code.

### `slog` {#slog}

[![slog][c~slog~docs~badge]][c~slog~docs] [![slog~crates.io][c~slog~crates.io~badge]][c~slog~crates.io] [![slog~github][c~slog~github~badge]][c~slog~github] [![slog~lib.rs][c~slog~lib.rs~badge]][c~slog~lib.rs]{{hi:slog}}{{hi:Structured}}{{hi:Log}}{{hi:Logging}}{{hi:Hierarchical}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}

[`slog`][c~slog~docs]↗{{hi:slog}} provides structured, extensible, composable logging.

Consider using [`tracing`][c~tracing~docs]↗{{hi:tracing}} instead, especially if you need [`async`][rust~book~async-await]↗{{hi:async}} support. [`slog`][c~slog~docs]↗{{hi:slog}} remains a stable, featureful and battle-tested library, used in many important projects.
[rust~book~async-await]: https://doc.rust-lang.org/book/ch17-00-async-await.html

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/other/slog.rs:example}}
```

### `log4rs` {#log4rs}

[![log4rs][c~log4rs~docs~badge]][c~log4rs~docs] [![log4rs~crates.io][c~log4rs~crates.io~badge]][c~log4rs~crates.io] [![log4rs~github][c~log4rs~github~badge]][c~log4rs~github] [![log4rs~lib.rs][c~log4rs~lib.rs~badge]][c~log4rs~lib.rs]{{hi:log4rs}}{{hi:Log4}}{{hi:Logger}}{{hi:Log}}{{hi:Logging}}

[`log4rs`][c~log4rs::append::file::FileAppender~docs]↗{{hi:log4rs}} [`log4rs`][c~log4rs::config::Config~docs]↗{{hi:log4rs}} [`log4rs`][c~log4rs::encode::pattern~docs]↗{{hi:log4rs}} [`log4rs`][c~log4rs~docs]↗{{hi:log4rs}} is a highly configurable multi-output logging implementation for the [`log`][c~log~docs]↗{{hi:log}} facade.

```rust,editable
use log::{error, info, warn};
use log4rs;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    info!("booting up");

    // ...
}
```

### `env_logger` {#env-logger}

[![env_logger][c~env_logger~docs~badge]][c~env_logger~docs] [![env_logger~crates.io][c~env_logger~crates.io~badge]][c~env_logger~crates.io] [![env_logger~github][c~env_logger~github~badge]][c~env_logger~github] [![env_logger~lib.rs][c~env_logger~lib.rs~badge]][c~env_logger~lib.rs]{{hi:env_logger}}{{hi:Log}}{{hi:Logger}}{{hi:Logging}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}

[`env_logger`][c~env_logger~docs]↗{{hi:env_logger}} is a logging implementation for [`log`][c~log~docs]↗{{hi:log}} which is configured via an environment variable. [`env_logger`][c~env_logger~docs]↗{{hi:env_logger}} makes sense when used in executables (binary projects). Libraries should use the [`log`][c~log~docs]↗{{hi:log}} crate instead.

```rust,editable
use log::info;

fn main() {
    env_logger::init();

    info!("starting up");

    // ...
}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write, organize together with the old log content.](https://github.com/john-cd/rust_howto/issues/649)
</div>
