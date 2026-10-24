<!-- TODO Reviewed ??-??-2026 -->

# Tracing

{{#include tracing.incl.md}}

The [`tracing`][c~tracing~docs]↗ ecosystem records structured events and spans while a program runs. Instrument code with the `tracing` API, then configure one subscriber to collect, filter, format, and write those records. The instrumentation API is separate from the output backend.

- **Instrumentation:** [`tracing`][c~tracing~docs]↗{{hi:tracing}} provides events, spans, and macros such as `event!`, `info!`, and `instrument`.
- **Collection and output:** [`tracing-subscriber`][c~tracing-subscriber~docs]↗{{hi:tracing-subscriber}} provides subscribers and composable layers to collect, filter, format, and write events and spans.
- **Filtering:** `tracing-subscriber` directives can select by level, target, span, and field.
- **Async instrumentation:** `Future::instrument` and `#[tracing::instrument]` scope spans to future polling.
- **File output:** [`tracing-appender`][tracing-appender~docs]↗ provides rolling file appenders and non-blocking writers.
- **Distributed tracing:** Exporters connect events to systems such as [`Jaeger`][jaegertracing~website]↗{{hi:Jaeger}} or [`Zipkin`][zipkin~website]↗{{hi:Zipkin}}. Context propagation across process or network boundaries requires an integration that encodes and extracts context.

## Log Trace Data to `stdout` {#tracing-basics}

[![tracing-subscriber][c~tracing-subscriber~docs~badge]][c~tracing-subscriber~docs]{{hi:tracing-subscriber}} [![tracing-subscriber~crates.io][c~tracing-subscriber~crates.io~badge]][c~tracing-subscriber~crates.io]

[![tracing][c~tracing~docs~badge]][c~tracing~docs]{{hi:tracing}} [![tracing~repo][c~tracing~repo~badge]][c~tracing~repo] [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}{{hi:Tracing}}

This example initializes a text-formatted subscriber that writes `tracing` events and spans to `stdout`.

Install and initialize a subscriber once near the application entry point. Libraries should emit spans and events but generally should not choose the application's global subscriber.

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber.rs:example}}
```

## Change the Log Level {#tracing-change-log-level}

[![tracing-subscriber][c~tracing-subscriber~docs~badge]][c~tracing-subscriber~docs]{{hi:tracing-subscriber}} [![tracing-subscriber~crates.io][c~tracing-subscriber~crates.io~badge]][c~tracing-subscriber~crates.io]

With `EnvFilter`, set the `RUST_LOG` environment variable to directives such as `info,my_crate=debug,[request]=trace`. A directive can select a level by target, span name, or span field. Treat environment-driven filters as configuration: validate and document the directives your application supports.

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber1.rs:example}}
```

## Log in JSON Format {#tracing-log-json}

[![tracing-subscriber][c~tracing-subscriber~docs~badge]][c~tracing-subscriber~docs]{{hi:tracing-subscriber}} [![tracing-subscriber~crates.io][c~tracing-subscriber~crates.io~badge]][c~tracing-subscriber~crates.io]

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber1b.rs:example}}
```

## Configure Tracing {#configure-tracing}

[![tracing-subscriber][c~tracing-subscriber~docs~badge]][c~tracing-subscriber~docs]{{hi:tracing-subscriber}} [![tracing-subscriber~crates.io][c~tracing-subscriber~crates.io~badge]][c~tracing-subscriber~crates.io]

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber_layers.rs:example}}
```

## Write Logs to Multiple Destinations Simultaneously {#tracing-multiple-destinations}

[![tracing-subscriber][c~tracing-subscriber~docs~badge]][c~tracing-subscriber~docs]{{hi:tracing-subscriber}} [![tracing-subscriber~crates.io][c~tracing-subscriber~crates.io~badge]][c~tracing-subscriber~crates.io]

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber2b.rs:example}}
```

The non-blocking writer returns a guard that must stay alive while the application writes logs; dropping it flushes and shuts down the worker.

## Configure Tracing at Runtime {#tracing-config-runtime}

[![tracing-subscriber][c~tracing-subscriber~docs~badge]][c~tracing-subscriber~docs]{{hi:tracing-subscriber}} [![tracing-subscriber~crates.io][c~tracing-subscriber~crates.io~badge]][c~tracing-subscriber~crates.io]

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber3.rs:example}}
```

## Change the Tracing Configuration at Runtime {#tracing-change-runtime}

[![tracing-subscriber][c~tracing-subscriber~docs~badge]][c~tracing-subscriber~docs]{{hi:tracing-subscriber}} [![tracing-subscriber~crates.io][c~tracing-subscriber~crates.io~badge]][c~tracing-subscriber~crates.io]

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber3b.rs:example}}
```

## Customize the Formatting of Log Lines {#custom-event-formatter}

[![tracing-subscriber][c~tracing-subscriber~docs~badge]][c~tracing-subscriber~docs]{{hi:tracing-subscriber}} [![tracing-subscriber~crates.io][c~tracing-subscriber~crates.io~badge]][c~tracing-subscriber~crates.io]{{hi:Custom event formatter}}

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber4.rs:example}}
```

## Log Events with `tracing` {#events}

[![tracing][c~tracing~docs~badge]][c~tracing~docs]{{hi:tracing}} [![tracing~repo][c~tracing~repo~badge]][c~tracing~repo] [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}{{hi:Tracing}}

An [`Event`][c~tracing::Event~docs]↗{{hi:Event}} signifies something that happened at a moment in time. `tracing`'s events are comparable to the log records emitted by unstructured logging code.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/tracing/tracing.rs:example}}
```

## Create then Enter a Span {#spans}

[![tracing][c~tracing~docs~badge]][c~tracing~docs]{{hi:tracing}} [![tracing~repo][c~tracing~repo~badge]][c~tracing~repo] [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}{{hi:Tracing}}

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/tracing/tracing_spans.rs:example}}
```

## Create and Enter a Span in a One-liner Using `entered` {#span-entered}

[![tracing][c~tracing~docs~badge]][c~tracing~docs]{{hi:tracing}} [![tracing~repo][c~tracing~repo~badge]][c~tracing~repo] [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}{{hi:Tracing}}

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/tracing/tracing_span_entered.rs:example}}
```

## Use Spans to Instrument Synchronous and Asynchronous Code {#span-instrument}

Holding the drop guard returned by `Span::enter`{{hi:Span::enter}} across [`await`][keyword~await]↗{{hi:await}} points can produce incorrect traces because the task may yield while the span remains entered on that thread. Use [`tracing::span::Span::in_scope`][c~tracing::span::Span::in_scope~docs]↗{{hi:tracing::span::Span::in_scope}} for synchronous work that ends before an `await`. For async work, use `Future::instrument` or `#[tracing::instrument]`; the span is entered while the future is polled and exited when it yields.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/tracing/tracing_span_in_scope.rs:example}}
```

## Add Tracing Spans to Functions {#add-tracing-spans-to-fn}

[![tracing][c~tracing~docs~badge]][c~tracing~docs]{{hi:tracing}} [![tracing~repo][c~tracing~repo~badge]][c~tracing~repo] [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}{{hi:Tracing}}

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/tracing/tracing_instrument.rs:example}}
```

## References {#references .skip}

- [Next steps with Tracing][c~tracing~next-steps]↗.
- [![tracing-journald][c~tracing-journald~docs~badge]][c~tracing-journald~docs]{{hi:tracing-journald}}
- [![tracing-journald~crates.io][c~tracing-journald~crates.io~badge]][c~tracing-journald~crates.io]
- [![tracing-journald~repo][c~tracing-journald~repo~badge]][c~tracing-journald~repo]
- [![tracing-journald~lib.rs][c~tracing-journald~lib.rs~badge]][c~tracing-journald~lib.rs]

[tracing-journald][c~tracing-journald~docs]↗ provides support for logging [`tracing`][c~tracing~crates.io]↗ events natively to [journald][journald]↗, preserving any structured information.

- [Tracing documentation][c~tracing~docs]↗.
- [Subscriber documentation][c~tracing-subscriber~docs]↗.
- [Async instrumentation][c~tracing~docs]↗.
- [Tracing appender documentation][tracing-appender~docs]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
