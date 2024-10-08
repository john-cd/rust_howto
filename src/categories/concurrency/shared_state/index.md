# Shared-State Concurrency

Channels are similar to single ownership, because once you transfer a value down a channel, you should no longer use that value. Shared memory concurrency is like multiple ownership: multiple threads can access the same memory location at the same time.

The Rust standard library provides smart pointer types, such as `Mutex<T>` and `Arc<T>`, that are safe to use in concurrent contexts.

{{#include index.incl.md}}

{{#include concurrent_data_structures.incl.md}}

## Mutex

[![std][std-badge]][std]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

Allow access to data from one thread at a time.

```rust,editable
{{#include ../../../../deps/tests/shared_state_mutex.rs}}
```

## Parking Lot

[![parking-lot][parking-lot-badge]][parking-lot]  [(crates.io)][parking-lot-crate]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

[Parking Lot][parking-lot]⮳ provides implementations of [`Mutex`][parking_lot::Mutex]⮳, [`RwLock`][parking_lot::RwLock]⮳, [`Condvar`][parking_lot::Condvar]⮳ and [`Once`][parking_lot::Once]⮳ that are smaller, faster and more flexible than those in the Rust standard library. It also provides a [`ReentrantMutex`][parking_lot::ReentrantMutex]⮳ type.

`std::sync::Mutex` works fine, but `Parking Lot` is faster.

```rust,editable,mdbook-runnable
{{#include ../../../../deps/tests/shared_state_parking_lot.rs}}
```

```rust,editable,mdbook-runnable
{{#include ../../../../deps/tests/shared_state_parking_lot2.rs}}
```

## Atomics

[![std][std-badge]][std]  [![crossbeam][crossbeam-badge]][crossbeam]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

Atomic types in [`std::sync::atomic`][std::sync::atomic]⮳ provide primitive shared-memory communication between threads, and are the building blocks of other concurrent types. It defines atomic versions of a select number of primitive types, including [`AtomicBool`][std::sync::atomic::AtomicBool]⮳, [`AtomicIsize`][std::sync::atomic::AtomicIsize]⮳, [`AtomicUsize`][std::sync::atomic::AtomicUsize]⮳, [`AtomicI8`][std::sync::atomic::AtomicI8]⮳, [`AtomicU16`][std::sync::atomic::AtomicU16]⮳, etc.

```rust,editable,mdbook-runnable
{{#include ../../../../deps/tests/shared_state_atomics.rs}}
```

The most common way to share an atomic variable is to put it into an [`Arc`][std::sync::Arc]⮳ (an atomically-reference-counted shared pointer).

[`crossbeam`][crossbeam]⮳ also offers [`AtomicCell`][crossbeam::atomic::AtomicCell]⮳, a thread-safe mutable memory location. This type is equivalent to [`Cell`][std::cell::Cell]⮳, except it can also be shared among multiple threads.

```rust,editable,mdbook-runnable
{{#include ../../../../deps/tests/shared_state_crossbeam.rs}}
```

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
