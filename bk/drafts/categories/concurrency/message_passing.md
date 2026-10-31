<!-- TODO Reviewed ??-??-2026 -->

# Message Passing and Channels

{{#include message_passing.incl.md}}

One approach to safe [concurrency][p~concurrency] is _message passing_{{hi:Message passing}}: threads communicate by sending messages{{hi:Messages}} through channels{{hi:Channels}} instead of sharing mutable data directly. The standard library's `std::sync::mpsc` provides an unbounded multi-producer, single-consumer channel; use `sync_channel` when producers need a bounded queue and back-pressure.

For multiple consumers and `select!`, [`crossbeam-channel`][c~crossbeam-channel~docs]↗{{hi:crossbeam-channel}} provides bounded and unbounded MPMC channels. [`flume`][c~flume~docs]↗{{hi:flume}} and [`kanal`][c~kanal~docs]↗{{hi:kanal}} offer sync and async channel APIs. Compare their behavior and workload fit rather than relying on unqualified speed claims.

Message passing in [`async`][book~rust-reference~async]↗{{hi:async}} programming is covered in a separate page: [async channels][p~async-channels]↗{{hi:Async channels}}.

## Multiple Producers, Single Consumer {#mpsc}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}{{hi:MPSC}}

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/message_passing/message_passing_mpsc.rs:example}}
```

## `crossbeam-channel` {#crossbeam-channel}

[![crossbeam-channel~website][c~crossbeam-channel~website~badge]][c~crossbeam-channel~website] [![crossbeam-channel][c~crossbeam-channel~docs~badge]][c~crossbeam-channel~docs] [![crossbeam-channel~crates.io][c~crossbeam-channel~crates.io~badge]][c~crossbeam-channel~crates.io] [![crossbeam-channel~repo][c~crossbeam-channel~repo~badge]][c~crossbeam-channel~repo] [![crossbeam-channel~lib.rs][c~crossbeam-channel~lib.rs~badge]][c~crossbeam-channel~lib.rs]{{hi:crossbeam-channel}}{{hi:Channel}}{{hi:Select}}{{hi:Mpmc}}{{hi:Golang}}{{hi:Message}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

[`crossbeam-channel`][c~crossbeam-channel~docs]↗{{hi:crossbeam-channel}} offers bounded and unbounded multi-producer, multi-consumer channels{{hi:Multi-consumer channels}} for message passing. Its `select!` macro waits for one of several channel operations to become ready.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/message_passing/message_passing_crossbeam_channel.rs:example}}
```

This example also uses `tick` and `after` to schedule periodic work and a timeout.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/message_passing/message_passing_crossbeam_channel_after_tick.rs:example}}
```

### `flume` {#flume}

{{#badges flume}}

The [`flume`][c~flume~docs]↗{{hi:flume}} crate is a library that provides multiple-producer, multiple-consumer (MPMC) channels. It is similar to the [`std::sync::mpsc`][c~std::sync::mpsc::channel~docs]↗{{hi:std::sync::mpsc}} module, but with additional features and improved performance. It is smaller and simpler than [`crossbeam-channel`][c~crossbeam-channel~docs]↗{{hi:crossbeam-channel}} and almost as fast.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/message_passing/flume.rs:example}}
```

### Handle Commands Once per Tick with `flume` {#flume-tick}

[![flume][c~flume~docs~badge]][c~flume~docs]{{hi:flume}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

A game loop, a UI frame or a control cycle handles pending commands once per _tick_ and must not wait for more. A bounded [`flume`][c~flume~docs]↗{{hi:flume}} channel fits: `send` blocks when the channel is full (back-pressure), `try_iter` drains only the messages already queued, and `recv_timeout` waits at most one tick for new work.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/message_passing/flume_tick.rs:example}}
```

## Wait-free Ring Buffers for Real-Time Threads {#spsc}

A real-time thread, such as an audio callback, must never wait on a lock or allocate memory: either can take unbounded time. A _single-producer, single-consumer_ (SPSC) ring buffer{{hi:Ring buffer}}{{hi:SPSC}} lets such a thread exchange data with the rest of the program. The buffer is allocated once, `push` and `pop` never block, and they return an error when the buffer is full or empty so the caller decides what to do.

Use an ordinary channel for commands and telemetry, and a ring buffer only for the hot path.

### `rtrb` {#rtrb}

{{#badges rtrb concurrency=Concurrency data-structures="Data structures"}}{{hi:Ring buffer}}{{hi:Wait-free}}{{hi:Real-time}}{{hi:Audio}}

[`rtrb`][c~rtrb~docs]↗{{hi:rtrb}} (real-time ring buffer) is a wait-free SPSC queue. The example sends samples from a producer thread and drains them in fixed-size blocks on the consumer side. It also shows what `push` and `pop` return when the buffer is full or empty.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/spsc/rtrb.rs:example}}
```

`drain_block` returns after at most one fixed-size block and fills missing samples with silence. Call it once per real-time callback. The example's outer loop is an ordinary test driver that yields while waiting for the producer; that waiting loop does not belong in the callback.

### `ringbuf` {#ringbuf}

{{#badges ringbuf concurrency=Concurrency data-structures="Data structures"}}{{hi:Ring buffer}}{{hi:Lock-free}}

[`ringbuf`][c~ringbuf~docs]↗{{hi:ringbuf}} is a lock-free SPSC ring buffer with a trait-based API (`Producer`, `Consumer`, `Split`).

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/spsc/ringbuf.rs:example}}
```

### `thread-priority` {#thread-priority}

{{#badges thread-priority concurrency=Concurrency os="Operating systems"}}{{hi:Scheduling}}{{hi:Real-time}}

A real-time thread often also needs a higher scheduling priority. [`thread-priority`][c~thread-priority~docs]↗{{hi:thread-priority}} sets it, either on the current thread (`ThreadPriority::Max.set_for_current()`) or when spawning with `ThreadBuilder`. The call is platform-dependent and may require privileges, so the example reports whether the request was granted and carries on at the normal priority otherwise.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/spsc/thread_priority.rs:example}}
```

## Choosing a Channel {#choosing-a-channel}

| Need                                                    | Use                                                                         |
| ------------------------------------------------------- | --------------------------------------------------------------------------- |
| Several producers, one consumer, no extra dependency    | [`std::sync::mpsc`][c~std::sync::mpsc::channel~docs]↗                       |
| Several consumers, `select!`, tickers and timeouts      | [`crossbeam-channel`][c~crossbeam-channel~docs]↗                            |
| A channel API that can be used from sync and async code | [`flume`][c~flume~docs]↗ or [`kanal`][c~kanal~docs]↗                        |
| A real-time thread that must not block or allocate      | an SPSC ring buffer: [`rtrb`][c~rtrb~docs]↗ or [`ringbuf`][c~ringbuf~docs]↗ |

## See Also {#see-also .skip}

- [Message passing (rust book)][book~rust~message-passing]↗{{hi:Message passing}}.

## Related Topics {#related-topics .skip}

- [[shared_state | Shared State]] for data that must be accessed by multiple threads.
- [[concurrent_data_structures | Concurrent Data Structures]] for shared maps and queues.
- [[_actors | Actors]] for message-driven state isolation.
- [[threadpool | Threadpools]] for dispatching independent work to reusable workers.
- [[async_channels | Async Channels]] for communication between async tasks.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
