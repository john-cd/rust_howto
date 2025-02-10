# Multithreading with the `crossbeam` crate

{{#include crossbeam.incl.md}}

## Spawn a short-lived thread {#spawn-a-short-lived-thread}

[![crossbeam][c-crossbeam-badge]][c-crossbeam] [![crossbeam-crates.io][c-crossbeam-crates.io-badge]][c-crossbeam-crates.io] [![crossbeam-github][c-crossbeam-github-badge]][c-crossbeam-github] [![crossbeam-lib.rs][c-crossbeam-lib.rs-badge]][c-crossbeam-lib.rs]{{hi:crossbeam}}{{hi:Garbage}}{{hi:Non-blocking}}{{hi:Rcu}}{{hi:Atomic}}{{hi:Lock-free}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}} [![cat-memory-management][cat-memory-management-badge]][cat-memory-management]{{hi:Memory management}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}{{hi:Short-lived thread}}

The example uses the [`crossbeam`][c-crossbeam]{{hi:crossbeam}}⮳ crate, which provides data structures and functions for concurrent and parallel programming{{hi:Parallel programming}}. [`crossbeam::thread::Scope::spawn`][c-crossbeam::thread::Scope::spawn]{{hi:crossbeam::thread::Scope::spawn}}⮳ spawns a new scoped thread that is guaranteed to terminate before returning from the closure that passed into [`crossbeam::scope`][c-crossbeam::scope]{{hi:crossbeam::scope}}⮳ function, meaning that you can reference data from the calling function.

This example splits the array in half and performs the work in separate threads.

```rust,editable
{{#include ../../../crates/ex/cats/concurrency/tests/crossbeam/crossbeam_spawn.rs:example}}
```

## Create a parallel pipeline {#parallel-pipeline}

[![crossbeam][c-crossbeam-badge]][c-crossbeam] [![crossbeam-crates.io][c-crossbeam-crates.io-badge]][c-crossbeam-crates.io] [![crossbeam-github][c-crossbeam-github-badge]][c-crossbeam-github] [![crossbeam-lib.rs][c-crossbeam-lib.rs-badge]][c-crossbeam-lib.rs]{{hi:crossbeam}}{{hi:Garbage}}{{hi:Non-blocking}}{{hi:Rcu}}{{hi:Atomic}}{{hi:Lock-free}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}} [![cat-memory-management][cat-memory-management-badge]][cat-memory-management]{{hi:Memory management}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

[![crossbeam-channel-website][c-crossbeam_channel-website-badge]][c-crossbeam_channel-website] [![crossbeam-channel][c-crossbeam_channel-badge]][c-crossbeam_channel] [![crossbeam-channel-crates.io][c-crossbeam_channel-crates.io-badge]][c-crossbeam_channel-crates.io] [![crossbeam-channel-github][c-crossbeam_channel-github-badge]][c-crossbeam_channel-github] [![crossbeam-channel-lib.rs][c-crossbeam_channel-lib.rs-badge]][c-crossbeam_channel-lib.rs]{{hi:crossbeam-channel}}{{hi:Channel}}{{hi:Select}}{{hi:Mpmc}}{{hi:Golang}}{{hi:Message}} [![cat-algorithms][cat-algorithms-badge]][cat-algorithms]{{hi:Algorithms}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}

This example uses the [`crossbeam`][c-crossbeam]{{hi:crossbeam}}⮳ and [`crossbeam-channel`][c-crossbeam_channel]{{hi:crossbeam-channel}}⮳ crates to create a parallel pipeline{{hi:Parallel pipeline}}, similar to that described in the [ZeroMQ guide][zero-mq-guide-website]⮳. There is a data source and a data sink, with data being processed by two worker threads in parallel on its way from the source to the sink.

We use bounded channels with a capacity of one using
[`crossbeam_channel::bounded`][c-crossbeam_channel::bounded]{{hi:crossbeam_channel::bounded}}⮳. The producer must be on its own thread because it produces messages faster than the workers can process them (since they sleep for half a second) - this means the producer blocks on the call to
[`crossbeam_channel::Sender::send`][c-crossbeam_channel::Sender::send]{{hi:crossbeam_channel::Sender::send}}⮳ for half a second until one of the workers processes the data in the channel. Also note that the data in the channel is consumed by whichever worker calls receive first, so each message is delivered to a single worker rather than both workers.

Reading from the channels via the iterator
[`crossbeam_channel::Receiver::iter`][c-crossbeam_channel::Receiver::iter]{{hi:crossbeam_channel::Receiver::iter}}⮳ method will block, either waiting for new messages or until the channel is closed. Because the channels were created within the [`crossbeam::scope`][c-crossbeam::scope]{{hi:crossbeam::scope}}⮳ we must manually close them via [`std::ops::Drop`][c-std::ops::Drop]{{hi:std::ops::Drop}}⮳ to prevent the entire program from blocking on the worker for-loops. You can think of the calls to [`std::ops::Drop`][c-std::ops::Drop]{{hi:std::ops::Drop}}⮳ as signaling that no more messages will be sent.

```rust,editable
{{#include ../../../crates/ex/cats/concurrency/tests/crossbeam/crossbeam_complex.rs:example}}
```

## Pass data between two threads {#pass-data-between-two-threads}

[![crossbeam][c-crossbeam-badge]][c-crossbeam] [![crossbeam-crates.io][c-crossbeam-crates.io-badge]][c-crossbeam-crates.io] [![crossbeam-github][c-crossbeam-github-badge]][c-crossbeam-github] [![crossbeam-lib.rs][c-crossbeam-lib.rs-badge]][c-crossbeam-lib.rs]{{hi:crossbeam}}{{hi:Garbage}}{{hi:Non-blocking}}{{hi:Rcu}}{{hi:Atomic}}{{hi:Lock-free}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}} [![cat-memory-management][cat-memory-management-badge]][cat-memory-management]{{hi:Memory management}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

[![crossbeam-channel-website][c-crossbeam_channel-website-badge]][c-crossbeam_channel-website] [![crossbeam-channel][c-crossbeam_channel-badge]][c-crossbeam_channel] [![crossbeam-channel-crates.io][c-crossbeam_channel-crates.io-badge]][c-crossbeam_channel-crates.io] [![crossbeam-channel-github][c-crossbeam_channel-github-badge]][c-crossbeam_channel-github] [![crossbeam-channel-lib.rs][c-crossbeam_channel-lib.rs-badge]][c-crossbeam_channel-lib.rs]{{hi:crossbeam-channel}}{{hi:Channel}}{{hi:Select}}{{hi:Mpmc}}{{hi:Golang}}{{hi:Message}} [![cat-algorithms][cat-algorithms-badge]][cat-algorithms]{{hi:Algorithms}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}

This example demonstrates the use of [`crossbeam_channel`][c-crossbeam_channel]{{hi:crossbeam_channel}}⮳ in a single producer, single consumer{{hi:Single producer, single consumer}} (SPSC) setting. We build off the `crossbeam spawn`{{hi:crossbeam spawn}}⮳ example by using [`crossbeam::scope`][c-crossbeam::scope]{{hi:crossbeam::scope}}⮳ and [`crossbeam::thread::Scope::spawn`][c-crossbeam::thread::Scope::spawn]{{hi:crossbeam::thread::Scope::spawn}}⮳ to manage the producer thread. Data is exchanged between the two threads using a [`crossbeam::scope`][c-crossbeam::scope]{{hi:crossbeam::scope}}⮳ channel, meaning there is no limit to the number of storable messages{{hi:Messages}}. The producer thread sleeps for half a second in between messages.

```rust,editable
{{#include ../../../crates/ex/cats/concurrency/tests/crossbeam/crossbeam_spsc.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[crossbeam: cleanup (P1)](https://github.com/john-cd/rust_howto/issues/259)
</div>
