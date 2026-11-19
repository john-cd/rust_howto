<!-- TODO Reviewed ??-??-2026. -->

# Utilities

{{#include async_utilities.incl.md}}

## Visualize the Structure of Asynchronous Tasks and Their Dependencies {#utilities}

[![await-tree][c~await-tree~docs~badge]][c~await-tree~docs]{{hi:Await tree}}
[![await-tree~crates.io][c~await-tree~crates.io~badge]][c~await-tree~crates.io]
[![await-tree~repo][c~await-tree~repo~badge]][c~await-tree~repo]
[![await-tree~lib.rs][c~await-tree~lib.rs~badge]][c~await-tree~lib.rs]
[![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

The [`await-tree`][c~await-tree~docs]↗{{hi:await-tree}} crate provides a convenient way to visualize the structure of asynchronous tasks and their dependencies.

An `async` task's future can call other futures and spawn child tasks, forming a tree of work that changes as tasks are polled, complete, or are cancelled. [`await-tree`][c~await-tree~docs]↗{{hi:await-tree}} can record and display this structure; its `instrument_await` attribute adds task context to selected futures.

Test this Cargo-backed example from `bk/crates` with `cargo test --locked -p asynchronous --example await_tree`.

```rust,editable,noplayground
{{#include ../../../crates/cats/asynchronous/examples/await_tree.rs:example}}
```

## Collect Futures That Complete Independently {#futures-unordered}

[`FuturesUnordered`][c~futures::stream::FuturesUnordered~docs]↗{{hi:FuturesUnordered}} polls a set of futures and yields each output as soon as that future completes. It is useful when work can proceed concurrently and the results do not need to be returned in submission order.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/futures_unordered.rs:example}}
```

## Cancel Related Tasks {#cancellation-token}

[`CancellationToken`][c~tokio_util::sync::CancellationToken~docs]↗{{hi:CancellationToken}} lets one task signal cancellation to other tasks. Child tokens are cancelled when their parent is cancelled, which is useful for coordinating shutdown across a group of tasks. See [Cancel a Tree of Tasks with `CancellationToken`][ex~asynchronous~cancellation-tree] for a runnable example that signals cancellation and waits for cleanup. For a server-wide shutdown signal, see [graceful shutdown][ex~asynchronous~graceful-shutdown].

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/cancellation_token.rs:example}}
```

Use the same crate's codec support to [decode length-prefixed TCP messages](../network-programming/server.md#length-delimited-frames).

[`async-lock`][c~async-lock~docs] provides executor-independent asynchronous locks and semaphores.

## Related Topics {#related-topics .skip}

- [Concurrency](../concurrency/index.md).
- [Development Tools - Profiling](../development-tools_profiling/index.md).
- [Async Traits](async_traits.md).
- [Async Traits][ex~asynchronous~async-traits].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
