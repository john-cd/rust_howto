# Logs

{{#include tracing.incl.md}}

[![tracing][c-tracing-badge]][c-tracing]  [![tracing-github][c-tracing-github-badge]][c-tracing-github]  [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]  [![cat-debugging][cat-debugging-badge]][cat-debugging]

[![tracing_subscriber][c-tracing_subscriber-badge]][c-tracing_subscriber]  [![tracing_subscriber-crates.io][c-tracing_subscriber-crates.io-badge]][c-tracing_subscriber-crates.io]

Add to `Cargo.toml`

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = "0.3"
```

## Initialization

### Basic tracing

{{hi:Tracing}}
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

### Configure a custom event formatter
{{hi:Custom event formatter}}

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

Holding the drop guard returned by `Span::enter`{{hi:Span::enter}} across `.await` points will result in incorrect traces. Use [`tracing::span::Span::in_scope`][c-tracing::span::Span::in_scope]{{hi:tracing::span::Span::in_scope}}⮳.

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

[![env_logger][c-env_logger-badge]][c-env_logger]

[![log][c-log-badge]][c-log]

[![log4rs][c-log4rs-badge]][c-log4rs]

Support for logging [`tracing`][c-tracing-crates.io]⮳ events natively to [journald][journald]⮳, preserving structured information: [tracing_journald][c-tracing_journald]⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
