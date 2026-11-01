<!-- TODO Reviewed ??-??-2026 -->

# Alternatives

{{#include tracing_alternatives.incl.md}}

[`tracing`][c~tracing~docs]↗{{hi:tracing}} is the usual starting point for new applications that need structured events, spans, or context that follows asynchronous work. The [`log`][c~log~docs]↗{{hi:log}} facade remains useful when a project needs simple level-based records or already depends on the logging ecosystem. These APIs solve related but different problems; choose one instrumentation API for each component and add a compatibility layer only when needed.

## Use the `log` Facade {#log}

{{#badges log development-tools::debugging=Debugging}}{{hi:Logging}}

[`log`][c~log~docs]↗{{hi:log}} defines a small facade for level-based records. The application selects one logging implementation, such as [`env_logger`][c~env_logger~docs]↗{{hi:env_logger}} for environment-configured console output or [`log4rs`][c~log4rs~docs]↗{{hi:log4rs}} for configurable appenders and formats. See [[log | Log Messages]] for tested recipes.

## Keep an Existing `slog` Integration {#slog}

{{#badges slog development-tools::debugging=Debugging}}{{hi:Structured}}{{hi:Logging}}

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
