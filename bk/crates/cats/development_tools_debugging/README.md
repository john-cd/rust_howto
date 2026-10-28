<!-- TODO Reviewed ??-??-2026 -->

# Development Tools Debugging

Examples of logging, tracing, and observability patterns using tracing, log4rs, and OpenTelemetry. It demonstrates integration with key ecosystem crates including `anyhow`, `axum`, `chrono`, `clap`, `env_logger`, `log`.

## Examples Covered

- **Debugging** (`examples/debugging/`): Demonstrates concepts like `backtrace`, `dbg_macro`.
- **Log** (`examples/log/`): Demonstrates concepts like `log_custom`, `log_custom_logger`, `log_debug`, `log_env_variable`, `log_error`, `log_mod`, `log_stdout`, `log_syslog`, `log_timestamp`.
- **Metrics** (`examples/metrics/`): Demonstrates concepts like `metrics_route`, `prometheus`, `record_metrics`.
- **Other** (`examples/other/`): Demonstrates concepts like `open_observe`, `open_telemetry`, `slog`, `type_name_of_val`.
- **Tracing** (`examples/tracing/`): Demonstrates concepts like `tracing`, `tracing_instrument`, `tracing_span_entered`, `tracing_span_in_scope`, `tracing_spans`, `tracing_subscriber`, `tracing_subscriber1`, `tracing_subscriber1b`, `tracing_subscriber2b`, `tracing_subscriber3`, `tracing_subscriber3b`, `tracing_subscriber4`, `tracing_subscriber_layers`.

## Usage

To check or run examples in this crate:

```console
cargo check --package development_tools_debugging --examples
cargo test --package development_tools_debugging
```

Code examples correspond to chapters in `bk/src/categories/development_tools_debugging` or related book sections.
