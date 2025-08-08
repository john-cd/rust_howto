# Asynchronous Programming

[![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

Crates to help you deal with events independently of the main program flow, using techniques like futures, promises, waiting, or eventing.

## Useful Crates

- Runtimes: [`tokio`][c~tokio~docs]↗{{hi:tokio}}, [`async-std`][c~async-std~docs]↗{{hi:async-std}}, [`smol`][c~smol~docs]↗{{hi:smol}}.
- Futures/Streams: [`futures`][c~futures~docs]↗{{hi:futures}}, [`tokio-stream`][c~tokio-stream~docs]↗{{hi:tokio-stream}}.
- Async I/O: [`tokio`][c~tokio~docs]↗{{hi:tokio}}, [`async-std`][c~async-std~docs]↗{{hi:async-std}}.

## Async Basics

{{#include async.incl.md}}

## Futures

{{#include futures.incl.md}}

## Tokio Runtime

{{#include tokio.incl.md}}

## Async Channels

{{#include async-channels.incl.md}}

## Async Traits

{{#include async-traits.incl.md}}

## Streams

{{#include streams.incl.md}}

## Calling Async from Blocking Code and Vice Versa

{{#include async_and_blocking.incl.md}}

## Async Utilities

{{#include async_utilities.incl.md}}

## Related Topics

### Networking / Websocket / HTTP

[`tokio-tungstenite`][c~tokio-tungstenite~docs]↗{{hi:tokio-tungstenite}}, [`hyper`][c~hyper~docs]↗{{hi:hyper}}, [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}}.

- [[network-programming | Network Programming]].
- [[web-programming_http-client | Web Programming HTTP Client]].
- [[web-programming_http-server | Web Programming HTTP Server]].
- [[web-programming_websocket | Web Programming Websocket]].

### Concurrency: `tokio::sync`, [`async-channel`][c~async-channel~docs]↗{{hi:async-channel}}

- [[concurrency | Concurrency]].
- [[concurrent_data_structures | Concurrent Data Structures]].
- [[async_channels | Async Channels]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/905)
</div>
