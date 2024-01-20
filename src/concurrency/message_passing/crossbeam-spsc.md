# Pass data between two threads

[![crossbeam-badge]][crossbeam] [![cat-concurrency-badge]][cat-concurrency]

This example demonstrates the use of [crossbeam-channel] in a single producer, single
consumer (SPSC) setting. We build off the [ex-crossbeam-spawn] example by using
[`crossbeam::scope`] and [`Scope::spawn`] to manage the producer thread. Data is
exchanged between the two threads using a [`crossbeam_channel::unbounded`]
channel, meaning there is no limit to the number of storeable messages. The
producer thread sleeps for half a second in between messages.

```rust,editable
{#include ../../../deps/examples/crossbeam-spsc.rs}
```

[crossbeam-channel]: https://docs.rs/crate/crossbeam-channel/
[ex-crossbeam-spawn]: ../thread/concurrency/threads.html#spawn-a-short-lived-thread
[`crossbeam::scope`]: https://docs.rs/crossbeam/*/crossbeam/fn.scope.html
[`Scope::spawn`]: https://docs.rs/crossbeam/*/crossbeam/thread/struct.Scope.html#method.spawn
[`crossbeam_channel::unbounded`]: https://docs.rs/crossbeam-channel/*/crossbeam_channel/fn.unbounded.html
