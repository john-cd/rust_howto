<!-- TODO Reviewed ??-??-2026 -->

# Message Passing and Channels

{{#include message_passing.incl.md}}

One increasingly popular approach to ensuring safe [concurrency][p~concurrency] is *message passing*{{hi:Message passing}}, where threads communicate by sending each other messages{{hi:Messages}} containing data. The Rust standard library provides *channels*{{hi:Channels}} for message passing that are safe to use in concurrent contexts{{hi:Concurrent contexts}}.

Message passing in [`async`][book~rust-reference~async]↗{{hi:async}} programming is covered in a separate page: [async channels][p~async-channels]↗{{hi:Async channels}}.

## Multiple Producers, Single Consumer {#mpsc}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}{{hi:MPSC}}

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/message_passing/message_passing_mpsc.rs:example}}
```

## `crossbeam-channel` {#crossbeam-channel}

[![crossbeam-channel~website][c~crossbeam-channel~website~badge]][c~crossbeam-channel~website] [![crossbeam-channel][c~crossbeam-channel~docs~badge]][c~crossbeam-channel~docs] [![crossbeam-channel~crates.io][c~crossbeam-channel~crates.io~badge]][c~crossbeam-channel~crates.io] [![crossbeam-channel~repo][c~crossbeam-channel~repo~badge]][c~crossbeam-channel~repo] [![crossbeam-channel~lib.rs][c~crossbeam-channel~lib.rs~badge]][c~crossbeam-channel~lib.rs]{{hi:crossbeam-channel}}{{hi:Channel}}{{hi:Select}}{{hi:Mpmc}}{{hi:Golang}}{{hi:Message}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

[`crossbeam-channel`][c~crossbeam-channel~docs]↗{{hi:crossbeam-channel}} offers multi-producer multi-consumer channels{{hi:Multi-consumer channels}} for message passing. The absolute fastest channel implementation available. Implements Go-like 'select' feature.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/message_passing/message_passing_crossbeam_channel.rs:example}}
```

Example using specialized channels for [`tickers`][c~tickers~docs]↗{{hi:tickers}}{{hi:Tickers}} and [`timeout`][c~reqwest::ClientBuilder::timeout~docs]↗{{hi:Timeout}}

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/message_passing/message_passing_crossbeam_channel_after_tick.rs:example}}
```

### `flume` {#flume}

[![flume][c~flume~docs~badge]][c~flume~docs]{{hi:flume}}
[![flume~crates.io][c~flume~crates.io~badge]][c~flume~crates.io]
[![flume~repo][c~flume~repo~badge]][c~flume~repo]
[![flume~lib.rs][c~flume~lib.rs~badge]][c~flume~lib.rs]

The [`flume`][c~flume~docs]↗{{hi:flume}} crate is a library that provides multiple-producer, multiple-consumer (MPMC) channels. It is similar to the [`std::sync::mpsc`][c~std::sync::mpsc::channel~docs]↗{{hi:std::sync::mpsc}} module, but with additional features and improved performance. It is smaller and simpler than [`crossbeam-channel`][c~crossbeam-channel~docs]↗{{hi:crossbeam-channel}} and almost as fast.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/message_passing/flume.rs:example}}
```

### Handle Commands Once per Tick with `flume` {#flume-tick}

[![flume][c~flume~docs~badge]][c~flume~docs]{{hi:flume}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

A game loop, a UI frame or a control cycle handles pending commands once per *tick* and must not wait for more. A bounded [`flume`][c~flume~docs]↗{{hi:flume}} channel fits: `send` blocks when the channel is full (back-pressure), `try_iter` drains only the messages already queued, and `recv_timeout` waits at most one tick for new work.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/message_passing/flume_tick.rs:example}}
```

## Wait-free Ring Buffers for Real-Time Threads {#spsc}

A real-time thread, such as an audio callback, must never wait on a lock or allocate memory: either can take unbounded time. A *single-producer, single-consumer* (SPSC) ring buffer{{hi:Ring buffer}}{{hi:SPSC}} lets such a thread exchange data with the rest of the program. The buffer is allocated once, `push` and `pop` never block, and they return an error when the buffer is full or empty so the caller decides what to do.

Use an ordinary channel for commands and telemetry, and a ring buffer only for the hot path.

### `rtrb` {#rtrb}

[![rtrb][c~rtrb~docs~badge]][c~rtrb~docs] [![rtrb~crates.io][c~rtrb~crates.io~badge]][c~rtrb~crates.io] [![rtrb~repo][c~rtrb~repo~badge]][c~rtrb~repo] [![rtrb~lib.rs][c~rtrb~lib.rs~badge]][c~rtrb~lib.rs]{{hi:rtrb}}{{hi:Ring buffer}}{{hi:Wait-free}}{{hi:Real-time}}{{hi:Audio}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

[`rtrb`][c~rtrb~docs]↗{{hi:rtrb}} (real-time ring buffer) is a wait-free SPSC queue. The example sends samples from a producer thread and drains them in fixed-size blocks on the consumer side. It also shows what `push` and `pop` return when the buffer is full or empty.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/spsc/rtrb.rs:example}}
```

### `ringbuf` {#ringbuf}

[![ringbuf][c~ringbuf~docs~badge]][c~ringbuf~docs] [![ringbuf~crates.io][c~ringbuf~crates.io~badge]][c~ringbuf~crates.io] [![ringbuf~repo][c~ringbuf~repo~badge]][c~ringbuf~repo] [![ringbuf~lib.rs][c~ringbuf~lib.rs~badge]][c~ringbuf~lib.rs]{{hi:ringbuf}}{{hi:Ring buffer}}{{hi:Lock-free}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

[`ringbuf`][c~ringbuf~docs]↗{{hi:ringbuf}} is a lock-free SPSC ring buffer with a trait-based API (`Producer`, `Consumer`, `Split`).

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/spsc/ringbuf.rs:example}}
```

A real-time thread often also needs a higher scheduling priority. [`thread-priority`][c~thread-priority~docs]↗{{hi:thread-priority}} sets it (`ThreadPriority::Max.set_for_current()`); the call is platform-dependent and may require privileges.

## Choosing a Channel {#choosing-a-channel}

| Need | Use |
|------|-----|
| Several producers, one consumer, no extra dependency | [`std::sync::mpsc`][c~std::sync::mpsc::channel~docs]↗ |
| Several consumers, `select!`, tickers and timeouts | [`crossbeam-channel`][c~crossbeam-channel~docs]↗ |
| A smaller dependency with a similar API, `try_iter`, sync and async use | [`flume`][c~flume~docs]↗ |
| A real-time thread that must not block or allocate | an SPSC ring buffer: [`rtrb`][c~rtrb~docs]↗ or [`ringbuf`][c~ringbuf~docs]↗ |

## See Also {#see-also .skip}

- [Message passing (rust book)][book~rust~message-passing]↗{{hi:Message passing}}.

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[message_passing: polish](https://github.com/john-cd/rust_howto/issues/264)
</div>
