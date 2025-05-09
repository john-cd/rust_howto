# Tracing

{{#include tracing.incl.md}}

Tracing refers to the process of capturing and recording information about the execution of a software program.

| Topic | Rust Crates |
|---|---|
| Tracing Framework | [`tracing`][c-tracing]⮳{{hi:tracing}} provides the core tracing functionality: spans, events, etc. |
| Event Logging | Handled by `tracing` using [macros][p-macros] like `event!` and `debug!`, `info!`, `warn!`, `error!`. |
| Span Management | Also handled by [`tracing`][c-tracing]⮳{{hi:tracing}} through its span API. Context propagation is built into tracing's span system. |
| Asynchronous Tracing | Supported by [`tracing`][c-tracing]⮳{{hi:tracing}} through its [asynchronous][p-asynchronous] span management. |
| Output and [Formatting][p-formatting] | [`tracing-subscriber`][c-tracing_subscriber]⮳{{hi:tracing-subscriber}} formats and outputs traces to various destinations. |
| Filtering | [`tracing-subscriber`][c-tracing_subscriber]⮳{{hi:tracing-subscriber}} allows filtering of traces based on level, target, etc. |
| Integration with other tools | [`tracing`][c-tracing]⮳{{hi:tracing}} is often used with other tools like `Jaeger` or `Zipkin` for distributed tracing. |

## Log Trace Data to `stdout` {#tracing-basics}

[![tracing_subscriber][c-tracing_subscriber-badge]][c-tracing_subscriber]{{hi:tracing_subscriber}} [![tracing_subscriber-crates.io][c-tracing_subscriber-crates.io-badge]][c-tracing_subscriber-crates.io]

[![tracing][c-tracing-badge]][c-tracing]{{hi:tracing}} [![tracing-github][c-tracing-github-badge]][c-tracing-github] [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}} [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}{{hi:Tracing}}

A one-liner allows recording of `tracing`'s `Events` and `Spans` by formatting them as text and logging them to `stdout`.

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_debugging/tests/tracing/tracing_subscriber.rs:example}}
```

## Change the Log Level {#tracing-change-log-level}

[![tracing_subscriber][c-tracing_subscriber-badge]][c-tracing_subscriber]{{hi:tracing_subscriber}} [![tracing_subscriber-crates.io][c-tracing_subscriber-crates.io-badge]][c-tracing_subscriber-crates.io]

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_debugging/tests/tracing/tracing_subscriber1.rs:example}}
```

## Log in JSON Format {#tracing-log-json}

[![tracing_subscriber][c-tracing_subscriber-badge]][c-tracing_subscriber]{{hi:tracing_subscriber}} [![tracing_subscriber-crates.io][c-tracing_subscriber-crates.io-badge]][c-tracing_subscriber-crates.io]

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_debugging/tests/tracing/tracing_subscriber1b.rs:example}}
```

## Configure Tracing {#configure-tracing}

[![tracing_subscriber][c-tracing_subscriber-badge]][c-tracing_subscriber]{{hi:tracing_subscriber}} [![tracing_subscriber-crates.io][c-tracing_subscriber-crates.io-badge]][c-tracing_subscriber-crates.io]

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_debugging/tests/tracing/tracing_subscriber2.rs:example}}
```

##  Write Logs to Multiple Destinations Simultaneously {#tracing-multiple-destinations}

[![tracing_subscriber][c-tracing_subscriber-badge]][c-tracing_subscriber]{{hi:tracing_subscriber}} [![tracing_subscriber-crates.io][c-tracing_subscriber-crates.io-badge]][c-tracing_subscriber-crates.io]

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_debugging/tests/tracing/tracing_subscriber2b.rs:example}}
```

## Configure Tracing at Runtime {#tracing-config-runtime}

[![tracing_subscriber][c-tracing_subscriber-badge]][c-tracing_subscriber]{{hi:tracing_subscriber}} [![tracing_subscriber-crates.io][c-tracing_subscriber-crates.io-badge]][c-tracing_subscriber-crates.io]

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_debugging/tests/tracing/tracing_subscriber3.rs:example}}
```

## Change the Tracing Configuration at Runtime {#tracing-change-runtime}

[![tracing_subscriber][c-tracing_subscriber-badge]][c-tracing_subscriber]{{hi:tracing_subscriber}} [![tracing_subscriber-crates.io][c-tracing_subscriber-crates.io-badge]][c-tracing_subscriber-crates.io]

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_debugging/tests/tracing/tracing_subscriber3b.rs:example}}
```

## Customize the Formatting of Log Lines {#custom-event-formatter}

[![tracing_subscriber][c-tracing_subscriber-badge]][c-tracing_subscriber]{{hi:tracing_subscriber}} [![tracing_subscriber-crates.io][c-tracing_subscriber-crates.io-badge]][c-tracing_subscriber-crates.io]{{hi:Custom event formatter}}

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_debugging/tests/tracing/tracing_subscriber4.rs:example}}
```

## Log Events with `tracing` {#events}

[![tracing][c-tracing-badge]][c-tracing]{{hi:tracing}} [![tracing-github][c-tracing-github-badge]][c-tracing-github] [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}} [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}{{hi:Tracing}}

An `Event` signifies something that happened at a moment in time. `tracing`'s `Events` are comparable to the log records emitted by unstructured logging code.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/tests/tracing/tracing.rs:example}}
```

## Create then Enter a Span {#spans}

[![tracing][c-tracing-badge]][c-tracing]{{hi:tracing}} [![tracing-github][c-tracing-github-badge]][c-tracing-github] [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}} [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}{{hi:Tracing}}

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/tests/tracing/tracing_spans.rs:example}}
```

## Create and Enter a Span in a One-liner Using `entered` {#span-entered}

[![tracing][c-tracing-badge]][c-tracing]{{hi:tracing}} [![tracing-github][c-tracing-github-badge]][c-tracing-github] [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}} [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}{{hi:Tracing}}

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/tests/tracing/tracing_span_entered.rs:example}}
```

## Use Spans to Instrument Synchronous and Asynchronous Code {#span-instrument}

Holding the drop guard returned by `Span::enter`{{hi:Span::enter}} across `.await` points will result in incorrect traces. Use [`tracing::span::Span::in_scope`][c-tracing::span::Span::in_scope]{{hi:tracing::span::Span::in_scope}}⮳ to make sure the span is exited before the `await` call. Alternatively, use `instrument` to make sure that the span is automatically exited and re-entered when a async function or block is awaited then resumed.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/tests/tracing/tracing_span_in_scope.rs:example}}
```

## Add Tracing Spans to Functions {#add-tracing-spans-to-fn}

[![tracing][c-tracing-badge]][c-tracing]{{hi:tracing}} [![tracing-github][c-tracing-github-badge]][c-tracing-github] [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}} [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}{{hi:Tracing}}

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/tests/tracing/tracing_instrument.rs:example}}
```

## See Also {#skip}

- [![tracing_journald][c-tracing_journald-badge]][c-tracing_journald]{{hi:tracing_journald}}
- [![tracing_journald-crates.io][c-tracing_journald-crates.io-badge]][c-tracing_journald-crates.io]
- [![tracing_journald-github][c-tracing_journald-github-badge]][c-tracing_journald-github]
- [![tracing_journald-lib.rs][c-tracing_journald-lib.rs-badge]][c-tracing_journald-lib.rs]

[tracing_journald][c-tracing_journald]⮳ provides support for logging [`tracing`][c-tracing-crates.io]⮳ events natively to [journald][journald]⮳, preserving any structured information.

## References {#skip}

- [Next steps with Tracing][c-tracing-next-steps]⮳.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/322)
FIXME tracing_subscriber2
rename files

- [tracing-appender](https://crates.io/crates/tracing-appender)

</div>
