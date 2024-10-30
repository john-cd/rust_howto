# Shared-State Concurrency

Channels{{hi:Channels}} are similar to single ownership, because once you transfer a value down a channel, you should no longer use that value. Shared memory concurrency{{hi:Shared memory concurrency}} is like multiple ownership: multiple threads can access the same memory location at the same time.

The Rust standard library provides smart pointer types, such as `Mutex<T>`{{hi:Mutex}} and `Arc<T>`{{hi:Arc}}, that are safe to use in concurrent contexts.

{{#include index.incl.md}}

{{#include concurrent_data_structures.incl.md}}

## Mutex

[![std][c-std-badge]][c-std]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

Allow access to data from one thread at a time.

```rust
{{#include ../../../../deps/tests/shared_state_mutex.rs}}
```

## Parking Lot

[![parking_lot][c-parking_lot-badge]][c-parking_lot]{{hi:parking_lot}}  [![parking_lot-crates.io][c-parking_lot-crates.io-badge]][c-parking_lot-crates.io]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

[`parking_lot`][c-parking_lot]{{hi:parking_lot}}⮳ provides implementations of [`parking_lot::Mutex`][c-parking_lot::Mutex]{{hi:parking_lot::Mutex}}⮳, [`parking_lot::RwLock`][c-parking_lot::RwLock]{{hi:parking_lot::RwLock}}⮳, [`parking_lot::Condvar`][c-parking_lot::Condvar]{{hi:parking_lot::Condvar}}⮳ and [`parking_lot::Once`][c-parking_lot::Once]{{hi:parking_lot::Once}}⮳ that are smaller, faster and more flexible than those in the Rust standard library. It also provides a [`parking_lot::ReentrantMutex`][c-parking_lot::ReentrantMutex]{{hi:parking_lot::ReentrantMutex}}⮳ type.

`std::sync::Mutex`{{hi:std::sync::Mutex}} works fine, but `parking_lot`{{hi:parking_lot}} is faster.

```rust,mdbook-runnable
{{#include ../../../../deps/tests/shared_state_parking_lot.rs}}
```

```rust,mdbook-runnable
{{#include ../../../../deps/tests/shared_state_parking_lot2.rs}}
```

## Atomics

[![std][c-std-badge]][c-std]  [![crossbeam][c-crossbeam-badge]][c-crossbeam]{{hi:crossbeam}}  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

Atomic types{{hi:Atomic types}} in [`std::sync::atomic`][c-std::sync::atomic]{{hi:std::sync::atomic}}⮳ provide primitive shared-memory communication between threads{{hi:Threads}}, and are the building blocks of other concurrent types. It defines atomic versions of a select number of primitive types, including [`std::sync::atomic::AtomicBool`][c-std::sync::atomic::AtomicBool]{{hi:std::sync::atomic::AtomicBool}}⮳, [`std::sync::atomic::AtomicIsize`][c-std::sync::atomic::AtomicIsize]{{hi:std::sync::atomic::AtomicIsize}}⮳, [`std::sync::atomic::AtomicUsize`][c-std::sync::atomic::AtomicUsize]{{hi:std::sync::atomic::AtomicUsize}}⮳, [`std::sync::atomic::AtomicI8`][c-std::sync::atomic::AtomicI8]{{hi:std::sync::atomic::AtomicI8}}⮳, [`std::sync::atomic::AtomicU16`][c-std::sync::atomic::AtomicU16]{{hi:std::sync::atomic::AtomicU16}}⮳, etc.

```rust,mdbook-runnable
{{#include ../../../../deps/tests/shared_state_atomics.rs}}
```

The most common way to share an atomic variable is to put it into an [`std::sync::Arc`][c-std::sync::Arc]{{hi:std::sync::Arc}}⮳ (an atomically-reference-counted shared pointer).

[`crossbeam`][c-crossbeam]{{hi:crossbeam}}⮳ also offers [`crossbeam::atomic::AtomicCell`][c-crossbeam::atomic::AtomicCell]{{hi:crossbeam::atomic::AtomicCell}}⮳, a thread-safe mutable memory location. This type is equivalent to [`std::cell::Cell`][c-std::cell::Cell]{{hi:std::cell::Cell}}⮳, except it can also be shared among multiple threads.

```rust,mdbook-runnable
{{#include ../../../../deps/tests/shared_state_crossbeam.rs}}
```

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">

parking_lot

Parking Lot is faster than `std::sync::Mutex`.
</div>
