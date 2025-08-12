# Tracing

{{#include tracing.incl.md}}

Tracing refers to the process of capturing and recording information about the execution of a software program.

| Topic | Rust Crates |
|---|---|
| Tracing Framework | [`tracing`][c~tracing~docs]↗{{hi:tracing}} provides the `core` tracing functionality: spans, events, etc. |
| Event Logging | Handled by [`tracing`]( )↗{{hi: }} using [macros][p~macros] like [`event!`](https://docs.rs/tracing/latest/tracing/macro.event.html)↗{{hi:event!}} and [`debug!`](https://docs.rs/tracing/latest/tracing/macro.debug.html)↗{{hi:debug!}}, [`info!`](https://docs.rs/tracing/latest/tracing/macro.info.html)↗{{hi:info!}}, [`warn!`](https://docs.rs/tracing/latest/tracing/macro.warn.html)↗{{hi:warn!}}, [`error!`](https://docs.rs/tracing/latest/tracing/macro.error.html)↗{{hi:error!}}. |
| Span Management | Also handled by [`tracing`][c~tracing~docs]↗{{hi:tracing}} through its span API. Context propagation is built into tracing's span system. |
| Asynchronous Tracing | Supported by [`tracing`][c~tracing~docs]↗{{hi:tracing}} through its [asynchronous][p~asynchronous] span management. |
| Output and [Formatting][p~formatting] | [`tracing-subscriber`][c~tracing-subscriber~docs]↗{{hi:tracing-subscriber}} formats and outputs traces to various destinations. |
| Filtering | [`tracing-subscriber`][c~tracing-subscriber~docs]↗{{hi:tracing-subscriber}} allows filtering of traces based on level, target, etc. |
| Integration with other tools | [`tracing`][c~tracing~docs]↗{{hi:tracing}} is often used with other tools like [`Jaeger`][jaegertracing~website]↗{{hi:Jaeger}} or [`Zipkin`][zipkin~website]↗{{hi:Zipkin}} for distributed tracing. |

## Log Trace Data to `stdout` {#tracing-basics}

[![tracing-subscriber][c~tracing-subscriber~docs~badge]][c~tracing-subscriber~docs]{{hi:tracing-subscriber}} [![tracing-subscriber~crates.io][c~tracing-subscriber~crates.io~badge]][c~tracing-subscriber~crates.io]

[![tracing][c~tracing~docs~badge]][c~tracing~docs]{{hi:tracing}} [![tracing~github][c~tracing~github~badge]][c~tracing~github] [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}{{hi:Tracing}}

A one-liner allows recording of `tracing`'s `Events` and `Spans` by formatting them as text and logging them to `stdout`.

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber.rs:example}}
```

## Change the Log Level {#tracing-change-log-level}

[![tracing-subscriber][c~tracing-subscriber~docs~badge]][c~tracing-subscriber~docs]{{hi:tracing-subscriber}} [![tracing-subscriber~crates.io][c~tracing-subscriber~crates.io~badge]][c~tracing-subscriber~crates.io]

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
{{#include ../../../crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber2.rs:example}}
```

## Write Logs to Multiple Destinations Simultaneously {#tracing-multiple-destinations}

[![tracing-subscriber][c~tracing-subscriber~docs~badge]][c~tracing-subscriber~docs]{{hi:tracing-subscriber}} [![tracing-subscriber~crates.io][c~tracing-subscriber~crates.io~badge]][c~tracing-subscriber~crates.io]

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber2b.rs:example}}
```

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

[![tracing][c~tracing~docs~badge]][c~tracing~docs]{{hi:tracing}} [![tracing~github][c~tracing~github~badge]][c~tracing~github] [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}{{hi:Tracing}}

An [`Event`]( )↗{{hi: }} signifies something that happened at a moment in time. `tracing`'s `Events` are comparable to the log records emitted by unstructured logging code.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/tracing/tracing.rs:example}}
```

## Create then Enter a Span {#spans}

[![tracing][c~tracing~docs~badge]][c~tracing~docs]{{hi:tracing}} [![tracing~github][c~tracing~github~badge]][c~tracing~github] [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}{{hi:Tracing}}

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/tracing/tracing_spans.rs:example}}
```

## Create and Enter a Span in a One-liner Using `entered` {#span-entered}

[![tracing][c~tracing~docs~badge]][c~tracing~docs]{{hi:tracing}} [![tracing~github][c~tracing~github~badge]][c~tracing~github] [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}{{hi:Tracing}}

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/tracing/tracing_span_entered.rs:example}}
```

## Use Spans to Instrument Synchronous and Asynchronous Code {#span-instrument}

Holding the drop guard returned by `Span::enter`{{hi:Span::enter}} across [`await`](https://doc.rust-lang.org/stable/std/keyword.await.html)↗{{hi:await}} points will result in incorrect traces. Use [`tracing::span::Span::in_scope`][c~tracing::span::Span::in_scope~docs]{{hi:tracing::span::Span::in_scope}}↗ to make sure the span is exited before the `await` call. Alternatively, use `instrument` to make sure that the span is automatically exited and re-entered when a async function or block is awaited then resumed.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/tracing/tracing_span_in_scope.rs:example}}
```

## Add Tracing Spans to Functions {#add-tracing-spans-to-fn}

[![tracing][c~tracing~docs~badge]][c~tracing~docs]{{hi:tracing}} [![tracing~github][c~tracing~github~badge]][c~tracing~github] [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}{{hi:Tracing}}

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/tracing/tracing_instrument.rs:example}}
```

## Related Topics {#related-topics}

- [![tracing-journald][c~tracing-journald~docs~badge]][c~tracing-journald~docs]{{hi:tracing-journald}}
- [![tracing-journald~crates.io][c~tracing-journald~crates.io~badge]][c~tracing-journald~crates.io]
- [![tracing-journald~github][c~tracing-journald~github~badge]][c~tracing-journald~github]
- [![tracing-journald~lib.rs][c~tracing-journald~lib.rs~badge]][c~tracing-journald~lib.rs]

[tracing-journald][c~tracing-journald~docs]↗ provides support for logging [`tracing`][c~tracing~crates.io]↗ events natively to [journald][journald]↗, preserving any structured information.

## References {#references}

- [Next steps with Tracing][c~tracing~next-steps]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/322)
FIXME tracing_subscriber2
rename files

- [tracing-appender][c~tracing-appender~crates.io]↗.

</div>
