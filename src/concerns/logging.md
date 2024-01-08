# Logs

[![tracing-badge]][tracing]

[Tokio tracing (GitHub)][tracing-github]⮳

[tracing-subscriber (crates)][tracing-subscriber-crate]⮳

Add to `Cargo.toml`

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = "0.3"
```

## Initialization

### Basic tracing

```rust,editable,ignore,noplayground
{{#include ../../deps/examples/tracing_subscriber.rs}}
```

### Combine layers

```rust,editable,ignore,noplayground
{{#include ../../deps/examples/tracing_subscriber2.rs}}
```

Or with a custom formatting layer

```rust,editable,ignore,noplayground
{{#include ../../deps/examples/tracing_subscriber3.rs}}
```

### Configure a custom event formatter

```rust,editable,ignore,noplayground
{{#include ../../deps/examples/tracing_subscriber4.rs}}
```

## Events

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/tracing.rs}}
```

## Spans

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/tracing_spans.rs}}
```

One-liner with `.entered()`:

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/tracing_span_entered.rs}}
```

Holding the drop guard returned by `Span::enter` across `.await` points will result in incorrect traces. Use `in_scope`

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/tracing_span_in_scope.rs}}
```

## Add tracing spans to functions

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/tracing_instrument.rs}}
```

## OpenTelemetry

[OpenTelemetry Rust documentation][opentelemetry-rust]⮳

[opentelemetry-rust]: https://opentelemetry.io/docs/instrumentation/rust/
{{#include ../links.md}}
