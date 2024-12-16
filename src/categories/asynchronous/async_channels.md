# Channels for use in async code

{{#include async_channels.incl.md}}

The most common form of synchronization{{hi:Synchronization}} in an async{{hi:async}} program is message passing{{hi:Message passing}}. Two tasks operate independently and send messages to each other to synchronize. Doing so has the advantage of avoiding shared state{{hi:Shared state}}. Message passing is implemented using async channels{{hi:Async channels}}.

Tokio's [`sync`][c-tokio-sync]⮳ module provides channels{{hi:Channels}} that work well with async code.

## OneShot {#oneshot}

[![tokio][c-tokio-badge]][c-tokio]{{hi:tokio}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}

[`tokio::sync::oneshot`][c-tokio::sync::oneshot]{{hi:tokio::sync::oneshot}}⮳ sends a single value from a single producer{{hi:Producer}} to a single consumer{{hi:Consumer}}. This channel{{hi:Channels}} is usually used to send the result of a computation to a waiter.

```rust,editable
{{#include ../../../deps/tests/categories/asynchronous/async_channels_oneshot.rs:example}}
```

Another example:

```rust,editable
{{#include ../../../deps/tests/categories/asynchronous/async_channels_oneshot2.rs:example}}
```

## Send messages from multiple producers to a single consumer {#mpsc}

[![tokio][c-tokio-badge]][c-tokio]{{hi:tokio}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}

```rust,editable
{{#include ../../../deps/tests/categories/asynchronous/async_channels_mpsc.rs:example}}
```

## Send messages from multiple producers to one of multiple consumers

[![async-channel][c-async_channel-badge]][c-async_channel] [![async-channel-crates.io][c-async_channel-crates.io-badge]][c-async_channel-crates.io] [![async-channel-github][c-async_channel-github-badge]][c-async_channel-github] [![async-channel-lib.rs][c-async_channel-lib.rs-badge]][c-async_channel-lib.rs]{{hi:async-channel}}{{hi:Chan}}{{hi:Futures}}{{hi:Mpsc}}{{hi:Spmc}}{{hi:Mpmc}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

`async-channel` offers two kinds of async multi-producer multi-consumer channel, where each message can be received by only one of all existing consumers.

- Bounded channel with limited capacity,
- Unbounded channel with unlimited capacity.

The Sender and Receiver sides are cloneable and can be shared among multiple threads.

When all Senders or all Receivers are dropped, the channel becomes closed. When a channel is closed, no more messages can be sent, but remaining messages can still be received. The channel can also be closed manually by calling `Sender::close()` or `Receiver::close()`.

```rust,editable
{{#include ../../../deps/tests/categories/asynchronous/async_channel.rs:example}}
```

## Broadcast messages from multiple producers to multiple consumers

[![postage][c-postage-badge]][c-postage]{{hi:postage}}
[![postage-crates.io][c-postage-crates.io-badge]][c-postage-crates.io]
[![postage-github][c-postage-github-badge]][c-postage-github]
[![postage-lib.rs][c-postage-lib.rs-badge]][c-postage-lib.rs]

`postage` is a feature-rich, portable async channel library, with different options than Tokio. `postage::broadcast` provides a lossless MPMC channel, which all receivers are guaranteed to receive each message.

```rust,editable
{{#include ../../../deps/tests/categories/asynchronous/postage.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P1 review
add other postage channels?

## `kanal`

Fast sync and async channel:

[![kanal][c-kanal-badge]][c-kanal]{{hi:kanal}}
[![kanal-crates.io][c-kanal-crates.io-badge]][c-kanal-crates.io]
[![kanal-github][c-kanal-github-badge]][c-kanal-github]
[![kanal-lib.rs][c-kanal-lib.rs-badge]][c-kanal-lib.rs]

```rust,editable
{{#include ../../../deps/tests/categories/asynchronous/kanal.rs:example}}
```

</div>
