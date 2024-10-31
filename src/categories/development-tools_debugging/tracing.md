# Logs

{{#include tracing.incl.md}}

[![tracing][c-tracing-badge]][c-tracing]{{hi:tracing}}  [![tracing-github][c-tracing-github-badge]][c-tracing-github]  [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}  [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}

[![tracing_subscriber][c-tracing_subscriber-badge]][c-tracing_subscriber]{{hi:tracing_subscriber}}  [![tracing_subscriber-crates.io][c-tracing_subscriber-crates.io-badge]][c-tracing_subscriber-crates.io]

Add to `Cargo.toml`

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = "0.3"
```

## Initialization

### Basic tracing

{{hi:Tracing}}

```rust,noplayground
{{#include ../../../deps/tests/cats/development_tools_debugging/tracing_subscriber.rs}}
```

### Combine layers

```rust,noplayground
{{#include ../../../deps/tests/cats/development_tools_debugging/tracing_subscriber2.rs}}
```

Or with a custom formatting layer

```rust,noplayground
{{#include ../../../deps/tests/cats/development_tools_debugging/tracing_subscriber3.rs}}
```

### Configure a custom event formatter

{{hi:Custom event formatter}}

```rust,noplayground
{{#include ../../../deps/tests/cats/development_tools_debugging/tracing_subscriber4.rs}}
```

## Events

```rust
{{#include ../../../deps/tests/cats/development_tools_debugging/tracing.rs}}
```

## Spans

```rust
{{#include ../../../deps/tests/cats/development_tools_debugging/tracing_spans.rs}}
```

One-liner with `.entered()`:

```rust
{{#include ../../../deps/tests/cats/development_tools_debugging/tracing_span_entered.rs}}
```

Holding the drop guard returned by `Span::enter`{{hi:Span::enter}} across `.await` points will result in incorrect traces. Use [`tracing::span::Span::in_scope`][c-tracing::span::Span::in_scope]{{hi:tracing::span::Span::in_scope}}⮳.

```rust
{{#include ../../../deps/tests/cats/development_tools_debugging/tracing_span_in_scope.rs}}
```

## Add tracing spans to functions

```rust
{{#include ../../../deps/tests/cats/development_tools_debugging/tracing_instrument.rs}}
```

## OpenTelemetry

[OpenTelemetry Rust documentation][opentelemetry-rust]⮳

## See also

[![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}

[![env_logger][c-env_logger-badge]][c-env_logger]{{hi:env_logger}}

[![log][c-log-badge]][c-log]{{hi:log}}

[![log4rs][c-log4rs-badge]][c-log4rs]{{hi:log4rs}}

Support for logging [`tracing`][c-tracing-crates.io]⮳ events natively to [journald][journald]⮳, preserving structured information: [tracing_journald][c-tracing_journald]{{hi:tracing_journald}}⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: [tracing-next-steps][c-tracing-next-steps]
</div>
