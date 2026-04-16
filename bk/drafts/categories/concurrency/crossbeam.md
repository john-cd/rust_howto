# Multithreading with the `crossbeam` Crate

{{#include crossbeam.incl.md}}

## Spawn a Short-lived Thread {#spawn-a-short-lived-thread}

[![crossbeam][c~crossbeam~docs~badge]][c~crossbeam~docs] [![crossbeam~crates.io][c~crossbeam~crates.io~badge]][c~crossbeam~crates.io] [![crossbeam~repo][c~crossbeam~repo~badge]][c~crossbeam~repo] [![crossbeam~lib.rs][c~crossbeam~lib.rs~badge]][c~crossbeam~lib.rs]{{hi:crossbeam}}{{hi:Garbage}}{{hi:Non-blocking}}{{hi:Rcu}}{{hi:Atomic}}{{hi:Lock-free}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}{{hi:Short-lived thread}}

The example uses the [`crossbeam`][c~crossbeam~docs]↗{{hi:crossbeam}} crate, which provides data structures and functions for concurrent and parallel programming{{hi:Parallel programming}}. [`crossbeam::thread::Scope::spawn`][c~crossbeam::thread::Scope::spawn~docs]↗{{hi:crossbeam::thread::Scope::spawn}} spawns a new scoped thread that is guaranteed to terminate before returning from the closure that passed into [`crossbeam::scope`][c~crossbeam::scope~docs]↗{{hi:crossbeam::scope}} function, meaning that we can reference data from the calling function.

This example splits the array in half and performs the work in separate threads.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/crossbeam/crossbeam_spawn.rs:example}}
```

## Create a Parallel Pipeline {#parallel-pipeline}

[![crossbeam][c~crossbeam~docs~badge]][c~crossbeam~docs] [![crossbeam~crates.io][c~crossbeam~crates.io~badge]][c~crossbeam~crates.io] [![crossbeam~repo][c~crossbeam~repo~badge]][c~crossbeam~repo] [![crossbeam~lib.rs][c~crossbeam~lib.rs~badge]][c~crossbeam~lib.rs]{{hi:crossbeam}}{{hi:Garbage}}{{hi:Non-blocking}}{{hi:Rcu}}{{hi:Atomic}}{{hi:Lock-free}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[![crossbeam-channel~website][c~crossbeam-channel~website~badge]][c~crossbeam-channel~website] [![crossbeam-channel][c~crossbeam-channel~docs~badge]][c~crossbeam-channel~docs] [![crossbeam-channel~crates.io][c~crossbeam-channel~crates.io~badge]][c~crossbeam-channel~crates.io] [![crossbeam-channel~repo][c~crossbeam-channel~repo~badge]][c~crossbeam-channel~repo] [![crossbeam-channel~lib.rs][c~crossbeam-channel~lib.rs~badge]][c~crossbeam-channel~lib.rs]{{hi:crossbeam-channel}}{{hi:Channel}}{{hi:Select}}{{hi:Mpmc}}{{hi:Golang}}{{hi:Message}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

This example uses the [`crossbeam`][c~crossbeam~docs]↗{{hi:crossbeam}} and [`crossbeam-channel`][c~crossbeam-channel~docs]↗{{hi:crossbeam-channel}} crates to create a parallel pipeline{{hi:Parallel pipeline}}, similar to that described in the [ZeroMQ guide][zero-mq-guide~website]↗. There is a data source and a data sink, with data being processed by two worker threads in parallel on its way from the source to the sink.

We use bounded channels with a capacity of one using [`crossbeam-channel::bounded`][c~crossbeam-channel::bounded~docs]↗{{hi:crossbeam-channel::bounded}}. The producer must be on its own thread because it produces messages faster than the workers can process them (since they sleep for half a second) - this means the producer blocks on the call to [`crossbeam-channel::Sender::send`][c~crossbeam-channel::Sender::send~docs]↗{{hi:crossbeam-channel::Sender::send}} for half a second until one of the workers processes the data in the channel. Also note that the data in the channel is consumed by whichever worker calls receive first, so each message is delivered to a single worker rather than both workers.

Reading from the channels via the iterator [`crossbeam-channel::Receiver::iter`][c~crossbeam-channel::Receiver::iter~docs]↗{{hi:crossbeam-channel::Receiver::iter}} method will block, either waiting for new messages or until the channel is closed. Because the channels were created within the [`crossbeam::scope`][c~crossbeam::scope~docs]↗{{hi:crossbeam::scope}} we must manually close them via [`std::ops::Drop`][c~std::ops::Drop~docs]↗{{hi:std::ops::Drop}} to prevent the entire program from blocking on the worker for-loops. You can think of the calls to [`std::ops::Drop`][c~std::ops::Drop~docs]↗{{hi:std::ops::Drop}} as signaling that no more messages will be sent.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/crossbeam/crossbeam_complex.rs:example}}
```

## Pass Data Between Two Threads {#pass-data-between-two-threads}

[![crossbeam][c~crossbeam~docs~badge]][c~crossbeam~docs] [![crossbeam~crates.io][c~crossbeam~crates.io~badge]][c~crossbeam~crates.io] [![crossbeam~repo][c~crossbeam~repo~badge]][c~crossbeam~repo] [![crossbeam~lib.rs][c~crossbeam~lib.rs~badge]][c~crossbeam~lib.rs]{{hi:crossbeam}}{{hi:Garbage}}{{hi:Non-blocking}}{{hi:Rcu}}{{hi:Atomic}}{{hi:Lock-free}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[![crossbeam-channel~website][c~crossbeam-channel~website~badge]][c~crossbeam-channel~website] [![crossbeam-channel][c~crossbeam-channel~docs~badge]][c~crossbeam-channel~docs] [![crossbeam-channel~crates.io][c~crossbeam-channel~crates.io~badge]][c~crossbeam-channel~crates.io] [![crossbeam-channel~repo][c~crossbeam-channel~repo~badge]][c~crossbeam-channel~repo] [![crossbeam-channel~lib.rs][c~crossbeam-channel~lib.rs~badge]][c~crossbeam-channel~lib.rs]{{hi:crossbeam-channel}}{{hi:Channel}}{{hi:Select}}{{hi:Mpmc}}{{hi:Golang}}{{hi:Message}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

This example demonstrates the use of [`crossbeam-channel`][c~crossbeam-channel~docs]↗{{hi:crossbeam-channel}} in a single producer, single consumer{{hi:Single producer, single consumer}} (SPSC) setting. We build off the `crossbeam spawn`{{hi:crossbeam spawn}} example by using [`crossbeam::scope`][c~crossbeam::scope~docs]↗{{hi:crossbeam::scope}} and [`crossbeam::thread::Scope::spawn`][c~crossbeam::thread::Scope::spawn~docs]↗{{hi:crossbeam::thread::Scope::spawn}} to manage the producer thread. Data is exchanged between the two threads using a [`crossbeam::scope`][c~crossbeam::scope~docs]↗{{hi:crossbeam::scope}} channel, meaning there is no limit to the number of storable messages{{hi:Messages}}. The producer thread sleeps for half a second in between messages.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/crossbeam/crossbeam_spsc.rs:example}}
```

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[crossbeam: cleanup](https://github.com/john-cd/rust_howto/issues/259)
</div>
