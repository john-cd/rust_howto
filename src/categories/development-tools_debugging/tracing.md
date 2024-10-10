# Logs

{{#include tracing.incl.md}}

[![tracing][tracing-badge]][c-tracing]  [![tracing-github][tracing-github-badge]][tracing-github]  [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]  [![cat-debugging][cat-debugging-badge]][cat-debugging]

[![tracing-subscriber][tracing-subscriber-badge]][tracing-subscriber]  [![tracing-subscriber-crates-io][tracing-subscriber-crate-badge]][tracing-subscriber-crates-io]

Add to `Cargo.toml`

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = "0.3"
```

## Initialization

### Basic {{i:tracing}}

```rust,editable,noplayground
{{#include ../../../deps/tests/tracing_subscriber.rs}}
```

### Combine layers

```rust,editable,noplayground
{{#include ../../../deps/tests/tracing_subscriber2.rs}}
```

Or with a custom formatting layer

```rust,editable,noplayground
{{#include ../../../deps/tests/tracing_subscriber3.rs}}
```

### Configure a {{i:custom event formatter}}

```rust,editable,noplayground
{{#include ../../../deps/tests/tracing_subscriber4.rs}}
```

## Events

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/tracing.rs}}
```

## Spans

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/tracing_spans.rs}}
```

One-liner with `.entered()`:

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/tracing_span_entered.rs}}
```

Holding the drop guard returned by `{{i:Span::enter}}` across `.await` points will result in incorrect traces. Use [`{{i:in_scope}}`][c-tracing::span::Span::in_scope]⮳.

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/tracing_span_in_scope.rs}}
```

## Add tracing spans to functions

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/tracing_instrument.rs}}
```

## OpenTelemetry

[OpenTelemetry Rust documentation][opentelemetry-rust]⮳

## See also

[![cat-debugging][cat-debugging-badge]][cat-debugging]

[![env_logger][env_logger-badge]][c-env_logger]

[![log][log-badge]][c-log]

[![log4rs][log4rs-badge]][c-log4rs]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
