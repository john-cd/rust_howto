# Asynchronous programming

[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}

Crates to help you deal with events independently of the main program flow, using techniques like futures, promises, waiting, or eventing.

## Async Basics

{{#include async.incl.md}}

## Futures

{{#include futures.incl.md}}

## Tokio Runtime

{{#include tokio.incl.md}}

## Async Channels

{{#include async_channels.incl.md}}

## Async Traits

{{#include async_traits.incl.md}}

## Streams

{{#include streams.incl.md}}

## Calling Async from Blocking Code and Vice Versa

{{#include async_and_blocking.incl.md}}

## Async Utilities

{{#include async_utilities.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[ P1 need titles](https://github.com/john-cd/rust_howto/issues/905)

Runtimes: [`tokio`][c-tokio]⮳{{hi:tokio}}, [`async-std`][c-async_std]⮳{{hi:async-std}}, [`smol`][c-smol]⮳{{hi:smol}}
Futures/Streams: [`futures`][c-futures]⮳{{hi:futures}}, [`tokio-stream`][c-tokio_stream]⮳{{hi:tokio-stream}}
Async I/O: [`tokio`][c-tokio]⮳{{hi:tokio}}, [`async-std`][c-async_std]⮳{{hi:async-std}}
Link to Networking / Websocket / HTTP: [`tokio-tungstenite`][c-tokio_tungstenite]⮳{{hi:tokio-tungstenite}}, [`hyper`][c-hyper]⮳{{hi:hyper}}, [`reqwest`][c-reqwest]⮳{{hi:reqwest}}
Link to Concurrency: `tokio::sync`, [`async-channel`][c-async_channel]⮳{{hi:async-channel}}

</div>
