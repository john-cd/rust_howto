<!-- TODO Reviewed ??-??-2026. -->

# Cancellation and Structured Concurrency

{{#include structured_concurrency.incl.md}}

An asynchronous program often starts a group of tasks that should live and die together: start them, bound how many run at once, cancel them cooperatively, and collect their results. This page shows how to do so with [`tokio`][c~tokio~docs]↗{{hi:tokio}}, [`tokio-util`][c~tokio-util~docs]↗{{hi:tokio-util}} and Tokio's channels.

From `bk/crates`, run the examples with Cargo.

```sh
cargo run --locked -p asynchronous --example structured_concurrency -- joinset_pool
cargo run --locked -p asynchronous --example structured_concurrency -- cancellation_tree
cargo run --locked -p asynchronous --example structured_concurrency -- watch_latest
cargo run --locked -p asynchronous --example structured_concurrency -- supervisor
cargo test --locked -p asynchronous --example structured_concurrency
```

## How Cancellation Works {#how-cancellation-works .skip}

- **Cancellation is cooperative.** A task can observe a signal such as a [`CancellationToken`][c~tokio_util::sync::CancellationToken~docs]↗{{hi:CancellationToken}}, finish its cleanup, and return.
- **Dropping a future cancels it.** The rest of its body never runs. Dropping a [`JoinHandle`][c~tokio::task::JoinHandle::abort~docs]↗{{hi:JoinHandle}} detaches the task. Call `abort()` to request cancellation.
- **`abort()` is abrupt.** Tokio drops the task's future when it can regain control. Synchronous destructors run, but the task cannot perform further async cleanup. Prefer a token when the task needs to flush data or complete another async cleanup step.
- **A panic or a cancellation is reported by the `JoinHandle`.** `JoinError::is_panic` and `JoinError::is_cancelled` tell them apart.

## Run a Bounded Pool of Tasks with `JoinSet` {#joinset-pool}

{{#badges tokio asynchronous=Asynchronous concurrency=Concurrency}}{{hi:JoinSet}}{{hi:Worker pool}}{{hi:Structured concurrency}}

[`JoinSet`][c~tokio::task::JoinSet~docs]↗{{hi:JoinSet}} owns a group of spawned tasks: dropping the set aborts them, and `join_next` yields each result as its task finishes. To bound concurrency, spawn up to a limit, then call `join_next` and spawn one more task each time a result arrives. The example collects results in completion order and distinguishes a panic from a cancellation.

The example builds its runtime with `start_paused(true)`. Tokio's clock then advances only when every task is waiting, so the sleeps take no real time and the completion order is deterministic.

```rust,ignore,noplayground
{{#include ../../../crates/cats/asynchronous/examples/structured_concurrency/joinset_pool.rs:example}}
```

## Cancel a Tree of Tasks with `CancellationToken` {#cancellation-tree}

{{#badges tokio-util asynchronous=Asynchronous concurrency=Concurrency}}{{hi:CancellationToken}}{{hi:TaskTracker}}{{hi:Graceful shutdown}}

Cancelling a parent token cancels its children, but cancelling a child does not affect its parent or siblings. Each worker waits on `token.cancelled()` in a `select!` next to its work. [`TaskTracker`][c~tokio_util::task::TaskTracker~docs]↗{{hi:TaskTracker}} lets the owner wait until a group of tasks has finished, which is what a graceful shutdown needs.

```rust,ignore,noplayground
{{#include ../../../crates/cats/asynchronous/examples/structured_concurrency/cancellation_tree.rs:example}}
```

For a smaller example of task ownership and aborting a spawned task, see [Pin Futures and Own Their Cancellation](../../standard-library/pinned_async_cancellation.md).

## Publish the Latest Value with `watch` {#watch-latest}

[![tokio][c~tokio~docs~badge]][c~tokio~docs]{{hi:tokio}}{{hi:watch}}{{hi:broadcast}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

Pick the Tokio channel by what the receivers need:

| Receivers need | Use |
| --- | --- |
| Only the latest value, such as configuration or state | [`watch`][c~tokio::sync::watch~docs]↗{{hi:watch}}: `send_replace` publishes, `changed().await` waits for a new value |
| Each event for each receiver, with lag reported if a receiver misses events | [`broadcast`][c~tokio::sync::broadcast~docs]↗{{hi:broadcast}} |
| A work queue where each message goes to one consumer | `mpsc` |

```rust,ignore,noplayground
{{#include ../../../crates/cats/asynchronous/examples/structured_concurrency/watch_latest.rs:example}}
```

## Restart a Failing Task with a Supervisor {#supervisor}

[![tokio][c~tokio~docs~badge]][c~tokio~docs]{{hi:tokio}}{{hi:Supervisor}}{{hi:Backoff}}{{hi:Retry}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

A supervisor runs a task, restarts it when it fails with capped exponential backoff, and stops when it is cancelled. Use it for a connection to a service that may go away.

```rust,ignore,noplayground
{{#include ../../../crates/cats/asynchronous/examples/structured_concurrency/supervisor.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
