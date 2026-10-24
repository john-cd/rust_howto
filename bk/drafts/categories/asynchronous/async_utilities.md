<!-- TODO Reviewed ??-??-2026 -->

# Utilities

{{#include async_utilities.incl.md}}

## Visualize the Structure of Asynchronous Tasks and Their Dependencies {#utilities}

[![await-tree][c~await-tree~docs~badge]][c~await-tree~docs]{{hi:Await tree}}
[![await-tree~crates.io][c~await-tree~crates.io~badge]][c~await-tree~crates.io]
[![await-tree~repo][c~await-tree~repo~badge]][c~await-tree~repo]
[![await-tree~lib.rs][c~await-tree~lib.rs~badge]][c~await-tree~lib.rs]
[![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

The [`await-tree`][c~await-tree~docs]↗{{hi:await-tree}} crate provides a convenient way to visualize the structure of asynchronous tasks and their dependencies.

"The [Futures][p~futures] in [Async][p~async] Rust can be arbitrarily composited or nested to achieve a variety of control flows. Assuming that the execution of each Future is represented as a node, then the [asynchronous][p~async] execution of an [async][p~async] task can be organized into a logical tree, which is constantly transformed over the polling, completion, and cancellation of [Futures][p~futures]. [`await-tree`][c~await-tree~docs]↗{{hi:await-tree}} allows developers to dump this execution tree at runtime, with the span of each [`Future`][c~std::future::Future~docs]↗{{hi:Future}} annotated by `instrument_await`."

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/await_tree.rs:example}}
```

## Collect Futures That Complete Independently {#futures-unordered}

[`FuturesUnordered`][c~futures::stream::FuturesUnordered~docs]↗{{hi:FuturesUnordered}} polls a set of futures and yields each output as soon as that future completes. It is useful when work can proceed concurrently and the results do not need to be returned in submission order.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/futures_unordered.rs:example}}
```

## Cancel Related Tasks {#cancellation-token}

[`CancellationToken`][c~tokio_util::sync::CancellationToken~docs]↗{{hi:CancellationToken}} lets one task signal cancellation to other tasks. Child tokens are cancelled when their parent is cancelled, which is useful for coordinating shutdown across a group of tasks. For a server-wide shutdown signal, see [graceful shutdown][ex~asynchronous~graceful-shutdown].

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/cancellation_token.rs:example}}
```

Since Rust 1.75, `async fn` is stable in traits. The [`async-trait`][c~async-trait~docs]↗{{hi:async-trait}} crate remains useful when a trait needs dynamic dispatch or an older MSRV; see [Async Traits][ex~asynchronous~async-traits].

## Related Topics {#related-topics .skip}

- [[concurrency | Concurrency]].
- [[development_tools | Development Tools]].
- [[development-tools_profiling | Development Tools Profiling]].
- [Async Traits][ex~asynchronous~async-traits].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
