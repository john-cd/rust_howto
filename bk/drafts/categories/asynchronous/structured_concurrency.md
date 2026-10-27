<!-- TODO Reviewed ??-??-2026 -->

# Cancellation and Structured Concurrency

{{#include structured_concurrency.incl.md}}

An asynchronous program often starts a group of tasks that should live and die together: start them, bound how many run at once, cancel them cooperatively, and collect their results. This page shows how to do so with [`tokio`][c~tokio~docs]↗{{hi:tokio}}, [`tokio-util`][c~tokio-util~docs]↗{{hi:tokio-util}} and Tokio's channels.

## How Cancellation Works {#how-cancellation-works .skip}

- **Cancellation is cooperative.** A task stops only at an `.await` where it observes a signal, such as a [`CancellationToken`][c~tokio_util::sync::CancellationToken~docs]↗{{hi:CancellationToken}}. A token lets the task clean up first.
- **Dropping a future cancels it.** The rest of its body never runs. Dropping a [`JoinHandle`][c~tokio::task::JoinHandle::abort~docs]↗{{hi:JoinHandle}} does _not_ cancel the task; call `abort()` for that.
- **`abort()` is abrupt.** It cancels the task at its next suspension point from the outside, without a chance to clean up. Prefer a token when the task holds resources.
- **A panic or a cancellation is reported by the `JoinHandle`.** `JoinError::is_panic` and `JoinError::is_cancelled` tell them apart.

## Run a Bounded Pool of Tasks with `JoinSet` {#joinset-pool}

[![tokio][c~tokio~docs~badge]][c~tokio~docs] [![tokio~crates.io][c~tokio~crates.io~badge]][c~tokio~crates.io] [![tokio~repo][c~tokio~repo~badge]][c~tokio~repo] [![tokio~lib.rs][c~tokio~lib.rs~badge]][c~tokio~lib.rs]{{hi:tokio}}{{hi:JoinSet}}{{hi:Worker pool}}{{hi:Structured concurrency}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

[`JoinSet`][c~tokio::task::JoinSet~docs]↗{{hi:JoinSet}} owns a group of spawned tasks: dropping the set aborts them, and `join_next` yields each result as its task finishes. To bound concurrency, spawn up to a limit, then call `join_next` and spawn one more task each time a result arrives. The example collects results in completion order and distinguishes a panic from a cancellation.

The example builds its runtime with `start_paused(true)`. Tokio's clock then advances only when every task is waiting, so the sleeps take no real time and the completion order is deterministic.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/structured_concurrency/joinset_pool.rs:example}}
```

## Cancel a Tree of Tasks with `CancellationToken` {#cancellation-tree}

[![tokio-util][c~tokio-util~docs~badge]][c~tokio-util~docs] [![tokio-util~crates.io][c~tokio-util~crates.io~badge]][c~tokio-util~crates.io] [![tokio-util~repo][c~tokio-util~repo~badge]][c~tokio-util~repo] [![tokio-util~lib.rs][c~tokio-util~lib.rs~badge]][c~tokio-util~lib.rs]{{hi:tokio-util}}{{hi:CancellationToken}}{{hi:TaskTracker}}{{hi:Graceful shutdown}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

Cancelling a parent token cancels its children, but cancelling a child does not affect its parent or siblings. Each worker waits on `token.cancelled()` in a `select!` next to its work. [`TaskTracker`][c~tokio_util::task::TaskTracker~docs]↗{{hi:TaskTracker}} lets the owner wait until a group of tasks has finished, which is what a graceful shutdown needs.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/structured_concurrency/cancellation_tree.rs:example}}
```

For a server-wide shutdown signal, see [graceful shutdown][ex~asynchronous~graceful-shutdown]; for a simpler token example, see [Cancel Related Tasks][ex~asynchronous~cancellation-token].

## Publish the Latest Value with `watch` {#watch-latest}

[![tokio][c~tokio~docs~badge]][c~tokio~docs]{{hi:tokio}}{{hi:watch}}{{hi:broadcast}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

Pick the Tokio channel by what the receivers need:

| Receivers need                                        | Use                                                                                                                  |
| ----------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| Only the latest value, such as configuration or state | [`watch`][c~tokio::sync::watch~docs]↗{{hi:watch}}: `send_replace` publishes, `changed().await` waits for a new value |
| Every event, accepting that a slow receiver can lag   | [`broadcast`][c~tokio::sync::broadcast~docs]↗{{hi:broadcast}}                                                        |
| A work queue where each message goes to one consumer  | `mpsc` (see [Async Channels][p~async-channels])                                                                      |

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/structured_concurrency/watch_latest.rs:example}}
```

## Restart a Failing Task with a Supervisor {#supervisor}

[![tokio][c~tokio~docs~badge]][c~tokio~docs]{{hi:tokio}}{{hi:Supervisor}}{{hi:Backoff}}{{hi:Retry}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

A supervisor runs a task, restarts it when it fails with capped exponential backoff, and stops when it is cancelled. Use it for a connection to a service that may go away.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/structured_concurrency/supervisor.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[async_channels | Async Channels]].
- [[async_utilities | Utilities]].
- [[tokio | Tokio Async Runtime]].
- [[concurrency | Concurrency]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [structured concurrency: review the new page](https://github.com/john-cd/rust_howto/issues/1920)
</div>
