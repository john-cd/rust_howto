<!-- TODO Reviewed ??-??-2026 -->

# Asynchronous Programming

[![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

Asynchronous programming lets a task suspend while waiting for I/O or another event, so an executor can run other ready tasks on the same thread. It is useful for applications that coordinate many I/O-bound operations; it does not make CPU-bound work faster by itself. Choose a runtime for its ecosystem and integration needs, then keep blocking work off its executor threads.

## Useful Crates

- Runtimes: [`tokio`][c~tokio~docs]↗{{hi:tokio}} is the broad, widely adopted runtime. [`smol`][c~smol~docs]↗{{hi:smol}} is a smaller alternative. [`async-std`][c~async-std~docs]↗{{hi:async-std}} is discontinued; prefer `smol` for new projects and plan a migration for existing users.
- Futures/Streams: [`futures`][c~futures~docs]↗{{hi:futures}}, [`tokio-stream`][c~tokio-stream~docs]↗{{hi:tokio-stream}}.
- Async I/O: use the I/O types provided by your selected runtime, such as [`tokio`][c~tokio~docs]↗{{hi:tokio}} or [`async-io`][c~async-io~docs]↗{{hi:async-io}}.

## Async Basics {#async-basics}

Learn how `Future`, `async`, and `.await` work and how an executor polls tasks in [Async Basics](async.md).

{{#include async.incl.md}}

## Futures {#futures}

Compose, pin, and poll futures with the standard future traits and the `futures` crate in [Futures](futures.md).

{{#include futures.incl.md}}

## Tokio Runtime {#tokio-runtime}

Set up tasks, timers, and I/O on Tokio, the runtime used by many Rust services, in [Tokio Runtime](tokio.md).

{{#include tokio.incl.md}}

## Async Channels {#async-channels}

Choose bounded or unbounded channels for communication between asynchronous tasks in [Async Channels](async_channels.md).

{{#include async-channels.incl.md}}

## Async Traits {#async-traits}

Expose asynchronous behavior through traits and choose an approach that fits the API's compatibility needs in [Async Traits](async_traits.md).

{{#include async-traits.incl.md}}

## Streams {#streams}

Process asynchronous sequences of values with `Stream` and stream combinators in [Streams](streams.md).

{{#include streams.incl.md}}

## Calling Async from Blocking Code and Vice Versa {#async-and-blocking}

Bridge synchronous and asynchronous code without blocking an executor thread in [Async and Blocking](async_and_blocking.md).

{{#include async_and_blocking.incl.md}}

## Async Utilities {#async-utilities}

Find utilities for timeouts, retries, task inspection, and other common async tasks in [Async Utilities](async_utilities.md).

{{#include async_utilities.incl.md}}

## Cancellation and Structured Concurrency {#structured-concurrency}

Manage task lifetimes, cancellation, and child-task completion as a unit in [Structured Concurrency](structured_concurrency.md).

{{#include structured_concurrency.incl.md}}

## Related Topics

### Networking / Websocket / HTTP

[`tokio-tungstenite`][c~tokio-tungstenite~docs]↗{{hi:tokio-tungstenite}}, [`hyper`][c~hyper~docs]↗{{hi:hyper}}, [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}}.

- [[network-programming | Network Programming]].
- [[web-programming_http-client | Web Programming HTTP Client]].
- [[web-programming_http-server | Web Programming HTTP Server]].
- [[web-programming_websocket | Web Programming Websocket]].

### Concurrency: `tokio::sync`, `async-channel`

- [[concurrency | Concurrency]].
- [[concurrent_data_structures | Concurrent Data Structures]].
- [[async_channels | Async Channels]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
