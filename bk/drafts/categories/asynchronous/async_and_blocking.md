<!-- TODO Reviewed ??-??-2026 -->
# Mixing Async and Blocking Code

{{#include async_and_blocking.incl.md}}

## Call Blocking Code from Async Code {#calling-blocking-from-async}

[![tokio][c~tokio~docs~badge]][c~tokio~docs]{{hi:tokio}} [![tokio~crates.io][c~tokio~crates.io~badge]][c~tokio~crates.io]
[![tokio~repo][c~tokio~repo~badge]][c~tokio~repo] [![tokio~lib.rs][c~tokio~lib.rs~badge]][c~tokio~lib.rs] [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}{{hi:Blocking code}}

- [Async][p~async] code should avoid long stretches of synchronous work. An [`.await`][book~rust-reference~await]↗{{hi:.await}} is a point where a future may yield to the executor; it does not move a blocking call to another thread.
- Don't carelessly mix [async][p~async]{{hi:async}} code and synchronous, blocking calls like `std::thread::sleep(Duration::from_secs(N));`.
- Prefer asynchronous I/O APIs inside async tasks. For a finite blocking operation or a synchronous I/O call, use [`tokio::task::spawn_blocking`][c~tokio::task::spawn_blocking~docs]↗{{hi:tokio::task::spawn_blocking}}. For CPU-heavy parallel work, use [`rayon`][c~rayon~docs]↗{{hi:rayon}} or another deliberately bounded CPU pool. Give a long-lived blocking loop a dedicated thread.

See the [Async: What is blocking?][blog~async-what-is-blocking]↗ blog post.

## Use `spawn_blocking` {#spawn-blocking}

[![tokio][c~tokio~docs~badge]][c~tokio~docs]{{hi:tokio}} [![tokio~crates.io][c~tokio~crates.io~badge]][c~tokio~crates.io]
[![tokio~repo][c~tokio~repo~badge]][c~tokio~repo] [![tokio~lib.rs][c~tokio~lib.rs~badge]][c~tokio~lib.rs] [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}{{hi:spawn_blocking}}

Use [`tokio::task::spawn_blocking`][c~tokio::task::spawn_blocking~docs]↗{{hi:tokio::task::spawn_blocking}} for finite synchronous work that cannot use an async API. Await its join handle to observe the result and any task panic. Started blocking closures cannot be stopped by aborting their handle, so keep them bounded and make cancellation behavior explicit. Do not use this pool for unlimited CPU work or a loop that is intended to run forever.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/call_blocking_from_async/call_blocking_from_async_spawn_blocking.rs:example}}
```

## Use the `rayon` Crate {#rayon}

[![rayon][c~rayon~docs~badge]][c~rayon~docs]{{hi:rayon}} [![rayon~crates.io][c~rayon~crates.io~badge]][c~rayon~crates.io]
[![rayon~repo][c~rayon~repo~badge]][c~rayon~repo] [![rayon~lib.rs][c~rayon~lib.rs~badge]][c~rayon~lib.rs]
[![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}{{hi:rayon}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

[`rayon`][c~rayon~docs]↗{{hi:rayon}}

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/call_blocking_from_async/call_blocking_from_async_rayon.rs:example}}
```

## Spawn a Dedicated Thread {#spawn-a-dedicated-thread}

[![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}{{hi:Dedicated thread}}

If a blocking operation{{hi:Blocking operation}} keeps running for the lifetime of the application, give it a dedicated thread{{hi:Dedicated thread}} and define how it receives shutdown signals and returns results.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/call_blocking_from_async/call_blocking_from_async_spawn_dedicated_thread.rs:example}}
```

## Call Async Code from Blocking Code {#call-async-from-blocking}

[Bridging with sync code][c~tokio~bridging-with-sync-code~website]↗ [![tokio][c~tokio~docs~badge]][c~tokio~docs]{{hi:tokio}} [![tokio~crates.io][c~tokio~crates.io~badge]][c~tokio~crates.io]
[![tokio~repo][c~tokio~repo~badge]][c~tokio~repo] [![tokio~lib.rs][c~tokio~lib.rs~badge]][c~tokio~lib.rs] [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}{{hi:spawn_blocking}}

In other cases, it may be easier to structure the application as largely synchronous{{hi:Synchronous code}}, with smaller or logically distinct asynchronous{{hi:Asynchronous}} portions. For instance, a GUI{{hi:GUI}} application might want to run the GUI code on the main thread and run a Tokio runtime next to it on another thread.

### Use the `futures` Executor {#futures-executor}

[![futures-executor][c~futures-executor~docs~badge]][c~futures-executor~docs]{{hi:futures-executor}}
[![futures-executor~crates.io][c~futures-executor~crates.io~badge]][c~futures-executor~crates.io]
[![futures-executor~repo][c~futures-executor~repo~badge]][c~futures-executor~repo]
[![futures-executor~lib.rs][c~futures-executor~lib.rs~badge]][c~futures-executor~lib.rs] [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

[`futures-executor`][c~futures-executor~docs]↗{{hi:futures-executor}} includes a minimal executor. The [`futures::executor::block_on`][c~futures_executor::block_on~docs]↗{{hi:futures::executor::block_on}} function can run a self-contained future from synchronous code. It does not provide a Tokio runtime, so use Tokio's runtime when the future needs Tokio I/O, timers, or other runtime services.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/call_async_from_blocking/call_async_from_blocking_futures_executor.rs:example}}
```

### Use the Tokio Runtime Directly {#tokio-runtime}

[![tokio][c~tokio~docs~badge]][c~tokio~docs]{{hi:tokio}} [![tokio~crates.io][c~tokio~crates.io~badge]][c~tokio~crates.io]
[![tokio~repo][c~tokio~repo~badge]][c~tokio~repo] [![tokio~lib.rs][c~tokio~lib.rs~badge]][c~tokio~lib.rs] [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}{{hi:spawn_blocking}}

When a mostly synchronous application needs async work, create or own a Tokio runtime at the boundary and call its `block_on` from a synchronous thread. Do not call `block_on` from inside an async task; await the future there instead.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/call_async_from_blocking/call_async_from_blocking_tokio_runtime.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[concurrency | Concurrency]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
