<!-- TODO Reviewed ??-??-2026 -->

# Alternatives

{{#include tracing_alternatives.incl.md}}

[`tracing`][c~tracing~docs]↗{{hi:tracing}} is the usual starting point for new applications that need structured events, spans, or context that follows asynchronous work. The [`log`][c~log~docs]↗{{hi:log}} facade remains useful when a project needs simple level-based records or already depends on the logging ecosystem. These APIs solve related but different problems; choose one instrumentation API for each component and add a compatibility layer only when needed.

## Use the `log` Facade {#log}

[![log][c~log~docs~badge]][c~log~docs] [![log~crates.io][c~log~crates.io~badge]][c~log~crates.io] [![log~repo][c~log~repo~badge]][c~log~repo] [![log~lib.rs][c~log~lib.rs~badge]][c~log~lib.rs]{{hi:log}}{{hi:Logging}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}

[`log`][c~log~docs]↗{{hi:log}} defines a small facade for level-based records. The application selects one logging implementation, such as [`env_logger`][c~env_logger~docs]↗{{hi:env_logger}} for environment-configured console output or [`log4rs`][c~log4rs~docs]↗{{hi:log4rs}} for configurable appenders and formats. See [[log | Log Messages]] for tested recipes.

## Keep an Existing `slog` Integration {#slog}

[![slog][c~slog~docs~badge]][c~slog~docs] [![slog~crates.io][c~slog~crates.io~badge]][c~slog~crates.io] [![slog~repo][c~slog~repo~badge]][c~slog~repo] [![slog~lib.rs][c~slog~lib.rs~badge]][c~slog~lib.rs]{{hi:slog}}{{hi:Structured}}{{hi:Logging}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}

[`slog`][c~slog~docs]↗{{hi:slog}} provides structured, composable logging. It can be a reasonable choice when it is already established in a project; evaluate its ecosystem and maintenance needs before choosing it for new work. Its current example is included from the book's tested example crate.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/other/slog.rs:example}}
```

## Bridge Existing `log` Calls to `tracing` {#log-compatibility}

The [`tracing`][c~tracing~docs]↗{{hi:tracing}} ecosystem provides compatibility crates for applications that need to collect records from existing [`log`][c~log~docs]↗{{hi:log}} instrumentation alongside tracing events. Use that bridge at the application boundary when migrating incrementally, and avoid initializing competing global subscribers or loggers.

## Related Topics {#related-topics .skip}

- [[tracing | Tracing]].
- [[log | Log Messages]].
- [[config_log | Logging Configuration]].
- [[distributed_telemetry | Distributed Telemetry]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
