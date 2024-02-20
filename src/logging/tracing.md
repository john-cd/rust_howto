# Logs

[![tracing-badge]][tracing] [![tracing-github][tracing-github-badge]][tracing-github]

[![tracing-subscriber-badge]][tracing-subscriber] [(crates.io)][tracing-subscriber-crate]

Add to `Cargo.toml`

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = "0.3"
```

## Initialization

### Basic tracing

```rust,editable,noplayground
{{#include ../../deps/examples/tracing_subscriber.rs}}
```

### Combine layers

```rust,editable,noplayground
{{#include ../../deps/examples/tracing_subscriber2.rs}}
```

Or with a custom formatting layer

```rust,editable,noplayground
{{#include ../../deps/examples/tracing_subscriber3.rs}}
```

### Configure a custom event formatter

```rust,editable,noplayground
{{#include ../../deps/examples/tracing_subscriber4.rs}}
```

## Events

```rust,editable,mdbook-runnable
{{#include ../../deps/examples/tracing.rs}}
```

## Spans

```rust,editable,mdbook-runnable
{{#include ../../deps/examples/tracing_spans.rs}}
```

One-liner with `.entered()`:

```rust,editable,mdbook-runnable
{{#include ../../deps/examples/tracing_span_entered.rs}}
```

Holding the drop guard returned by `Span::enter` across `.await` points will result in incorrect traces. Use `in_scope`

```rust,editable,mdbook-runnable
{{#include ../../deps/examples/tracing_span_in_scope.rs}}
```

## Add tracing spans to functions

```rust,editable,mdbook-runnable
{{#include ../../deps/examples/tracing_instrument.rs}}
```

## OpenTelemetry

[OpenTelemetry Rust documentation][opentelemetry-rust]⮳

## See also

[![env_logger-badge]][env_logger]

[![log-badge]][log]

[![log4rs-badge]][log4rs]

{{#include ../refs/link-refs.md}}
