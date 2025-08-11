# Channels for Use in Async Code

{{#include async_channels.incl.md}}

The most common form of synchronization{{hi:Synchronization}} in an async{{hi:async}} program is message passing{{hi:Message passing}}. Two tasks operate independently and send messages to each other to synchronize. Doing so has the advantage of avoiding shared state{{hi:Shared state}}. Message passing is implemented using async channels{{hi:Async channels}}.

## `tokio`'s Async Channels {#tokio}

[![tokio~website][c~tokio~website~badge]][c~tokio~website] [![tokio][c~tokio~docs~badge]][c~tokio~docs] [![tokio~crates.io][c~tokio~crates.io~badge]][c~tokio~crates.io] [![tokio~github][c~tokio~github~badge]][c~tokio~github] [![tokio~lib.rs][c~tokio~lib.rs~badge]][c~tokio~lib.rs]{{hi:tokio}}{{hi:Io}}{{hi:Async}}{{hi:Non-blocking}}{{hi:Futures}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

Tokio's [`sync`][c~tokio::sync~docs]↗ module provides channels{{hi:Channels}} that work well with async code.

### OneShot {#oneshot}

[`tokio::sync::oneshot`][c~tokio::sync::oneshot~docs]{{hi:tokio::sync::oneshot}}↗ sends a single value from a single producer{{hi:Producer}} to a single consumer{{hi:Consumer}}. This channel{{hi:Channels}} is usually used to send the result of a computation to a waiter.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async_channels/async_channels_oneshot.rs:example}}
```

Another example:

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async_channels/async_channels_oneshot2.rs:example}}
```

## Send Messages from Multiple Producers to a Single Consumer {#mpsc}

[![tokio][c~tokio~docs~badge]][c~tokio~docs]{{hi:tokio}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

[`tokio`][c~tokio~docs]↗{{hi:tokio}}

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async_channels/async_channels_mpsc.rs:example}}
```

## Send Messages from Multiple Producers to One of Multiple Consumers {#mpmc}

[![async-channel][c~async-channel~docs~badge]][c~async-channel~docs] [![async-channel~crates.io][c~async-channel~crates.io~badge]][c~async-channel~crates.io] [![async-channel~github][c~async-channel~github~badge]][c~async-channel~github] [![async-channel~lib.rs][c~async-channel~lib.rs~badge]][c~async-channel~lib.rs]{{hi:async-channel}}{{hi:Chan}}{{hi:Futures}}{{hi:Mpsc}}{{hi:Spmc}}{{hi:Mpmc}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

[`async-channel`][c~async-channel~docs]↗{{hi:async-channel}} offers two kinds of async multi-producer multi-consumer channel, where each message can be received by only one of all existing consumers.

- Bounded channel with limited capacity,
- Unbounded channel with unlimited capacity.

The Sender and Receiver sides are cloneable and can be shared among multiple threads.

When all Senders or all Receivers are dropped, the channel becomes closed. When a channel is closed, no more messages can be sent, but remaining messages can still be received. The channel can also be closed manually by calling [`Sender::close()`]( ){{hi: }} or `Receiver::close()`.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async_channels/async_channel.rs:example}}
```

## Broadcast Messages from Multiple Producers to Multiple Consumers {#broadcast}

[![postage][c~postage~docs~badge]][c~postage~docs]{{hi:postage}}
[![postage~crates.io][c~postage~crates.io~badge]][c~postage~crates.io]
[![postage~github][c~postage~github~badge]][c~postage~github]
[![postage~lib.rs][c~postage~lib.rs~badge]][c~postage~lib.rs]

[`postage`][c~postage::broadcast~docs]↗{{hi:postage}} [`postage`][c~postage~docs]↗{{hi:postage}} is a feature-rich, portable [async][p~async] channel library, with different options than [Tokio][p~tokio]. [`postage::broadcast`][c~postage::broadcast~docs]↗{{hi:postage::broadcast}} provides a lossless MPMC channel, which all receivers are guaranteed to receive each message.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async_channels/postage.rs:example}}
```

## `kanal` {#kanal}

[![kanal][c~kanal~docs~badge]][c~kanal~docs]{{hi:kanal}}
[![kanal~crates.io][c~kanal~crates.io~badge]][c~kanal~crates.io]
[![kanal~github][c~kanal~github~badge]][c~kanal~github]
[![kanal~lib.rs][c~kanal~lib.rs~badge]][c~kanal~lib.rs]

[`kanal`][c~kanal~docs]↗{{hi:kanal}} offers fast sync and async channels:

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async_channels/kanal.rs:example}}
```

## Related Topics {#related-topics}

- [[concurrency | Concurrency]].
- [[concurrent_data_structures | Concurrent Data Structures]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[async_channels: review](https://github.com/john-cd/rust_howto/issues/215) add other [`postage`][c~postage~docs]↗{{hi:postage}} channels?
</div>
