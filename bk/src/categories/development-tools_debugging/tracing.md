# Tracing

{{#include tracing.incl.md}}

Tracing refers to the process of capturing and recording information about the execution of a software program.

| Topic | Rust Crates |
|---|---|
| Tracing Framework | `tracing` provides the core tracing functionality, spans, events, etc. |
| Span Management | Handled by `tracing` through its span API. |
| Event Logging | Also handled by tracing using [macros][p-macros] like `event!` and `debug!`, `info!`, `warn!`, `error!`. |
| Context Propagation | Built into tracing's span system. |
| Instrumentation | Often done with [macros][p-macros] provided by tracing or other instrumentation libraries. |
| Output and [Formatting][p-formatting] | `tracing-subscriber` formats and outputs traces to various destinations. |
| Filtering | `tracing-subscriber` allows filtering of traces based on level, target, etc. |
| Asynchronous Tracing | Supported by `tracing` through its [asynchronous][p-asynchronous] span management. |
| Integration with other tools | `tracing` is often used with other tools like `Jaeger` or `Zipkin` for distributed tracing. |

[![tracing][c-tracing-badge]][c-tracing]{{hi:tracing}} [![tracing-github][c-tracing-github-badge]][c-tracing-github] [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}} [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}

[![tracing_subscriber][c-tracing_subscriber-badge]][c-tracing_subscriber]{{hi:tracing_subscriber}} [![tracing_subscriber-crates.io][c-tracing_subscriber-crates.io-badge]][c-tracing_subscriber-crates.io]

Add to [`Cargo.toml`][book-cargo-cargo-toml]⮳{{hi:Cargo.toml}}.

```toml
[dependencies]
tracing = "0.1.41" # Or latest version
tracing-subscriber = "0.3"
```

## Initialize the Logger {#initialization}

### Enable Basic Tracing {#basic-tracing}

{{hi:Tracing}}

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_debugging/tests/tracing/tracing_subscriber.rs:example}}
```

### Combine Layers {#combine-layers}

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_debugging/tests/tracing/tracing_subscriber2.rs:example}}
```

Or with a custom [formatting][p-formatting] layer

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_debugging/tests/tracing/tracing_subscriber3.rs:example}}
```

### Configure a Custom Event Formatter {#custom-event-formatter}

{{hi:Custom event formatter}}

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_debugging/tests/tracing/tracing_subscriber4.rs:example}}
```

## Events {#events}

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/tests/tracing/tracing.rs:example}}
```

## Spans {#spans}

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/tests/tracing/tracing_spans.rs:example}}
```

One-liner with `entered`:

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/tests/tracing/tracing_span_entered.rs:example}}
```

Holding the drop guard returned by `Span::enter`{{hi:Span::enter}} across `.await` points will result in incorrect traces. Use [`tracing::span::Span::in_scope`][c-tracing::span::Span::in_scope]{{hi:tracing::span::Span::in_scope}}⮳.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/tests/tracing/tracing_span_in_scope.rs:example}}
```

## Add Tracing Spans to Functions {#add-tracing-spans-to-fn}

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/tests/tracing/tracing_instrument.rs:example}}
```

## See Also {#related-crates}

[![tracing_journald][c-tracing_journald-badge]][c-tracing_journald]{{hi:tracing_journald}}
[![tracing_journald-crates.io][c-tracing_journald-crates.io-badge]][c-tracing_journald-crates.io]
[![tracing_journald-github][c-tracing_journald-github-badge]][c-tracing_journald-github]
[![tracing_journald-lib.rs][c-tracing_journald-lib.rs-badge]][c-tracing_journald-lib.rs]

[tracing_journald][c-tracing_journald]⮳ provides support for logging [`tracing`][c-tracing-crates.io]⮳ events natively to [journald][journald]⮳, preserving any structured information.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[tracing: [tracing-next-steps][c-tracing-next-steps]](https://github.com/john-cd/rust_howto/issues/322)
[tracing-next-steps][c-tracing-next-steps]
</div>
