# Shared-State Concurrency

{{i:Channels}} are similar to single ownership, because once you transfer a value down a channel, you should no longer use that value. {{i:Shared memory concurrency}} is like multiple ownership: multiple threads can access the same memory location at the same time.

The Rust standard library provides smart pointer types, such as `{{i:Mutex}}<T>` and `{{i:Arc}}<T>`, that are safe to use in concurrent contexts.

{{#include index.incl.md}}

{{#include concurrent_data_structures.incl.md}}

## Mutex

[![std][std-badge]][std]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

Allow access to data from one thread at a time.

```rust,editable
{{#include ../../../../deps/tests/shared_state_mutex.rs}}
```

## Parking Lot

[![parking-lot][parking-lot-badge]][parking-lot]  [![parking-lot-crates-io][parking-lot-crate-badge]][parking-lot-crates-io]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

[`{{i:Parking Lot}}`][parking-lot]⮳ provides implementations of [`{{i:Mutex}}`][parking_lot::Mutex]⮳, [`{{i:RwLock}}`][parking_lot::RwLock]⮳, [`{{i:Condvar}}`][parking_lot::Condvar]⮳ and [`{{i:Once}}`][parking_lot::Once]⮳ that are smaller, faster and more flexible than those in the Rust standard library. It also provides a [`{{i:ReentrantMutex}}`][parking_lot::ReentrantMutex]⮳ type.

`{{i:std::sync::Mutex}}` works fine, but `{{i:Parking Lot}}` is faster.

```rust,editable,mdbook-runnable
{{#include ../../../../deps/tests/shared_state_parking_lot.rs}}
```

```rust,editable,mdbook-runnable
{{#include ../../../../deps/tests/shared_state_parking_lot2.rs}}
```

## Atomics

[![std][std-badge]][std]  [![crossbeam][crossbeam-badge]][crossbeam]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

{{i:Atomic types}} in [`{{i:std::sync::atomic}}`][std::sync::atomic]⮳ provide primitive shared-memory communication between {{i:threads}}, and are the building blocks of other concurrent types. It defines atomic versions of a select number of primitive types, including [`{{i:AtomicBool}}`][std::sync::atomic::AtomicBool]⮳, [`{{i:AtomicIsize}}`][std::sync::atomic::AtomicIsize]⮳, [`{{i:AtomicUsize}}`][std::sync::atomic::AtomicUsize]⮳, [`{{i:AtomicI8}}`][std::sync::atomic::AtomicI8]⮳, [`{{i:AtomicU16}}`][std::sync::atomic::AtomicU16]⮳, etc.

```rust,editable,mdbook-runnable
{{#include ../../../../deps/tests/shared_state_atomics.rs}}
```

The most common way to share an atomic variable is to put it into an [`{{i:Arc}}`][std::sync::Arc]⮳ (an atomically-reference-counted shared pointer).

[`{{i:crossbeam}}`][crossbeam]⮳ also offers [`{{i:AtomicCell}}`][crossbeam::atomic::AtomicCell]⮳, a thread-safe mutable memory location. This type is equivalent to [`{{i:Cell}}`][std::cell::Cell]⮳, except it can also be shared among multiple threads.

```rust,editable,mdbook-runnable
{{#include ../../../../deps/tests/shared_state_crossbeam.rs}}
```

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
