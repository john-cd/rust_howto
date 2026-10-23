<!-- TODO Reviewed ??-??-2026 -->
# Channels for Use in Async Code

{{#include async_channels.incl.md}}

The most common form of synchronization{{hi:Synchronization}} in an async{{hi:async}} program is message passing{{hi:Message passing}}. Two tasks operate independently and send messages to each other to synchronize. Doing so has the advantage of avoiding shared state{{hi:Shared state}}. Message passing is implemented using async channels{{hi:Async channels}}.

Choose a channel by its delivery contract, not only by its name:

- A **bounded** channel applies backpressure when its buffer is full. The sender waits or handles a full result, which limits queued work and memory.
- An **unbounded** channel accepts messages without waiting for capacity. A slow consumer can let the queue grow until the process runs out of memory.
- A **work queue** gives each message to one consumer. A **broadcast** sends a copy to each subscribed receiver. A **watch** channel retains only the latest value.

Do not call a blocking receive operation from an async task. A blocking call occupies an executor worker and can prevent unrelated tasks from making progress.

## `tokio`'s Async Channels {#tokio}

[![tokio~website][c~tokio~website~badge]][c~tokio~website] [![tokio][c~tokio~docs~badge]][c~tokio~docs] [![tokio~crates.io][c~tokio~crates.io~badge]][c~tokio~crates.io] [![tokio~repo][c~tokio~repo~badge]][c~tokio~repo] [![tokio~lib.rs][c~tokio~lib.rs~badge]][c~tokio~lib.rs]{{hi:tokio}}{{hi:Io}}{{hi:Async}}{{hi:Non-blocking}}{{hi:Futures}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

Tokio's [`sync`][c~tokio::sync~docs]↗ module provides channels{{hi:Channels}} designed to work with async tasks. Its main channel types have distinct semantics:

- `oneshot` carries one value from one sender to one receiver, often as a task's reply.
- `mpsc` carries many values from one or more senders to one receiver. A bounded `mpsc::channel` applies backpressure; use it for a worker queue or a stream of commands.
- `broadcast` delivers each value to every active receiver. Its buffer is bounded; a receiver that falls too far behind must handle a lag notification.
- `watch` lets receivers observe the latest state. Intermediate updates may be overwritten, so it is not an event log.

Use these channels to communicate between tasks without sharing a lock-protected value. A channel does not make a message durable: decide what should happen when a task exits or drops its receiver.

### OneShot {#oneshot}

[`tokio::sync::oneshot`][c~tokio::sync::oneshot~docs]↗{{hi:tokio::sync::oneshot}} sends a single value from one producer{{hi:Producer}} to one consumer{{hi:Consumer}}. Use it to return a result from a spawned task or reply to one request. Sending consumes the sender; if the receiver is dropped, the send or receive reports that the other endpoint is gone.

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

[`tokio::sync::mpsc`][c~tokio::sync~docs]↗ provides a multi-producer, single-consumer queue. The example uses a bounded channel with capacity 100; choose a capacity based on how much work may safely wait in memory.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async_channels/async_channels_mpsc.rs:example}}
```

## Send Messages from Multiple Producers to One of Multiple Consumers {#mpmc}

[![async-channel][c~async-channel~docs~badge]][c~async-channel~docs] [![async-channel~crates.io][c~async-channel~crates.io~badge]][c~async-channel~crates.io] [![async-channel~repo][c~async-channel~repo~badge]][c~async-channel~repo] [![async-channel~lib.rs][c~async-channel~lib.rs~badge]][c~async-channel~lib.rs]{{hi:async-channel}}{{hi:Chan}}{{hi:Futures}}{{hi:Mpsc}}{{hi:Spmc}}{{hi:Mpmc}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

[`async-channel`][c~async-channel~docs]↗{{hi:async-channel}} provides bounded and unbounded multi-producer, multi-consumer channels. Each message is received by one consumer, which makes a bounded channel useful as a shared work queue. Use `bounded(capacity)` to limit queued messages; `unbounded()` has no capacity limit and needs an explicit memory-growth policy.

Sender and receiver handles are cloneable and can be shared among tasks or threads.

When all senders or all receivers are dropped, the channel becomes closed. A closed channel accepts no new messages, but queued values may still be received. Use a bounded channel unless an unbounded queue is a deliberate, measured choice.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async_channels/async_channel.rs:example}}
```

## Broadcast Messages from Multiple Producers to Multiple Consumers {#broadcast}

[![postage][c~postage~docs~badge]][c~postage~docs]{{hi:postage}}
[![postage~crates.io][c~postage~crates.io~badge]][c~postage~crates.io]
[![postage~repo][c~postage~repo~badge]][c~postage~repo]
[![postage~lib.rs][c~postage~lib.rs~badge]][c~postage~lib.rs]

[`postage`][c~postage~docs]↗ includes async channel types such as [`postage::broadcast`][c~postage::broadcast~docs]↗{{hi:postage::broadcast}}. Consult its documentation for the buffer, backpressure, and receiver-lifecycle guarantees that apply to the selected channel.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async_channels/postage.rs:example}}
```

## `kanal` {#kanal}

[![kanal][c~kanal~docs~badge]][c~kanal~docs]{{hi:kanal}}
[![kanal~crates.io][c~kanal~crates.io~badge]][c~kanal~crates.io]
[![kanal~repo][c~kanal~repo~badge]][c~kanal~repo]
[![kanal~lib.rs][c~kanal~lib.rs~badge]][c~kanal~lib.rs]

[`kanal`][c~kanal~docs]↗{{hi:kanal}} offers fast sync and async channels:

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async_channels/kanal.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[concurrency | Concurrency]].
- [[concurrent_data_structures | Concurrent Data Structures]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[async_channels: review](https://github.com/john-cd/rust_howto/issues/215) add other [`postage`][c~postage~docs]↗{{hi:postage}} channels?
</div>
