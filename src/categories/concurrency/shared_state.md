# Shared-State Concurrency

{{#include shared_state.incl.md}}

Channels{{hi:Channels}} are similar to single ownership, because once you transfer a value down a channel, you should no longer use that value. Shared memory concurrency{{hi:Shared memory concurrency}} is like multiple ownership: multiple threads can access the same memory location at the same time.

The Rust standard library provides smart pointer types, such as `Mutex<T>`{{hi:Mutex}} and `Arc<T>`{{hi:Arc}}, that are safe to use in concurrent contexts.

## Maintain global mutable state {#global-mutable-state}

[![lazy_static][c-lazy_static-badge]][c-lazy_static] [![lazy_static-crates.io][c-lazy_static-crates.io-badge]][c-lazy_static-crates.io] [![lazy_static-github][c-lazy_static-github-badge]][c-lazy_static-github] [![lazy_static-lib.rs][c-lazy_static-lib.rs-badge]][c-lazy_static-lib.rs]{{hi:lazy_static}}{{hi:Macro}}{{hi:Lazy}}{{hi:Static}} [![cat-memory-management][cat-memory-management-badge]][cat-memory-management]{{hi:Memory management}} [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}{{hi:Global mutable state}}

Declare global state using [`lazy static`][c-lazy_static]{{hi:lazy_static}}{{hi:Lazy static}}. [`lazy static`][c-lazy_static]{{hi:lazy_static}}⮳ creates a globally available `static ref` which requires a [`std::sync::Mutex`][c-std::sync::Mutex]{{hi:std::sync::Mutex}}⮳ to allow mutation (also see [`std::sync::RwLock`][c-std::sync::RwLock]{{hi:std::sync::RwLock}}⮳). The [`std::sync::Mutex`][c-std::sync::Mutex]{{hi:std::sync::Mutex}}⮳ wrap ensures the state cannot be simultaneously accessed by multiple threads, preventing race conditions. A [`std::sync::MutexGuard`][c-std::sync::MutexGuard]{{hi:std::sync::MutexGuard}}⮳ must be acquired to read or mutate the value stored in a [`std::sync::Mutex`][c-std::sync::Mutex]{{hi:std::sync::Mutex}}⮳.

```rust,editable
{{#include ../../../deps/tests/categories/concurrency/global_mut_state.rs:example}}
```

## Mutex {#mutex}

[![std][c-std-badge]][c-std] [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}{{hi:Mutex}}

Allow access to data from one thread at a time.

```rust,editable
{{#include ../../../deps/tests/categories/concurrency/shared_state_mutex.rs:example}}
```

## Parking Lot {#parking-lot}

[![parking_lot][c-parking_lot-badge]][c-parking_lot] [![parking_lot-crates.io][c-parking_lot-crates.io-badge]][c-parking_lot-crates.io] [![parking_lot-github][c-parking_lot-github-badge]][c-parking_lot-github] [![parking_lot-lib.rs][c-parking_lot-lib.rs-badge]][c-parking_lot-lib.rs]{{hi:parking_lot}}{{hi:Mutex}}{{hi:Thread}}{{hi:Rwlock}}{{hi:Condvar}}{{hi:Once}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

More compact and efficient implementations of the standard synchronization primitives.

[`parking_lot`][c-parking_lot]{{hi:parking_lot}}⮳ provides implementations of [`parking_lot::Mutex`][c-parking_lot::Mutex]{{hi:parking_lot::Mutex}}⮳, [`parking_lot::RwLock`][c-parking_lot::RwLock]{{hi:parking_lot::RwLock}}⮳, [`parking_lot::Condvar`][c-parking_lot::Condvar]{{hi:parking_lot::Condvar}}⮳ and [`parking_lot::Once`][c-parking_lot::Once]{{hi:parking_lot::Once}}⮳ that are smaller, faster and more flexible than those in the Rust standard library. It also provides a [`parking_lot::ReentrantMutex`][c-parking_lot::ReentrantMutex]{{hi:parking_lot::ReentrantMutex}}⮳ type.

`std::sync::Mutex`{{hi:std::sync::Mutex}} works fine, but `parking_lot`{{hi:parking_lot}} is faster.

```rust,editable
{{#include ../../../deps/tests/categories/concurrency/shared_state_parking_lot.rs:example}}
```

```rust,editable
{{#include ../../../deps/tests/categories/concurrency/shared_state_parking_lot2.rs:example}}
```

## Atomics {#atomics}

[![std][c-std-badge]][c-std] [![crossbeam][c-crossbeam-badge]][c-crossbeam]{{hi:crossbeam}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}{{hi:Atomics}}

Atomic types{{hi:Atomic types}} in [`std::sync::atomic`][c-std::sync::atomic]{{hi:std::sync::atomic}}⮳ provide primitive shared-memory communication between threads{{hi:Threads}}, and are the building blocks of other concurrent types. It defines atomic versions of a select number of primitive types, including [`std::sync::atomic::AtomicBool`][c-std::sync::atomic::AtomicBool]{{hi:std::sync::atomic::AtomicBool}}⮳, [`std::sync::atomic::AtomicIsize`][c-std::sync::atomic::AtomicIsize]{{hi:std::sync::atomic::AtomicIsize}}⮳, [`std::sync::atomic::AtomicUsize`][c-std::sync::atomic::AtomicUsize]{{hi:std::sync::atomic::AtomicUsize}}⮳, [`std::sync::atomic::AtomicI8`][c-std::sync::atomic::AtomicI8]{{hi:std::sync::atomic::AtomicI8}}⮳, [`std::sync::atomic::AtomicU16`][c-std::sync::atomic::AtomicU16]{{hi:std::sync::atomic::AtomicU16}}⮳, etc.

```rust,editable
{{#include ../../../deps/tests/categories/concurrency/shared_state_atomics.rs:example}}
```

The most common way to share an atomic variable is to put it into an [`std::sync::Arc`][c-std::sync::Arc]{{hi:std::sync::Arc}}⮳ (an atomically-reference-counted shared pointer).

[`crossbeam`][c-crossbeam]{{hi:crossbeam}}⮳ also offers [`crossbeam::atomic::AtomicCell`][c-crossbeam::atomic::AtomicCell]{{hi:crossbeam::atomic::AtomicCell}}⮳, a thread-safe mutable memory location. This type is equivalent to [`std::cell::Cell`][c-std::cell::Cell]{{hi:std::cell::Cell}}⮳, except it can also be shared among multiple threads.

```rust,editable
{{#include ../../../deps/tests/categories/concurrency/shared_state_crossbeam.rs:example}}
```

## `arc-swap` {#arc-swap}

[![arc-swap][c-arc_swap-badge]][c-arc_swap] [![arc-swap-crates.io][c-arc_swap-crates.io-badge]][c-arc_swap-crates.io] [![arc-swap-github][c-arc_swap-github-badge]][c-arc_swap-github] [![arc-swap-lib.rs][c-arc_swap-lib.rs-badge]][c-arc_swap-lib.rs]{{hi:arc-swap}}{{hi:Arc}}{{hi:Atomic}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}} [![cat-memory-management][cat-memory-management-badge]][cat-memory-management]{{hi:Memory management}}

The `ArcSwap` type is a container for an `Arc` that can be changed atomically. Semantically, it is similar to `Atomic<Arc<T>>` (if there was such a thing) or `RwLock<Arc<T>>` (but without the need for the locking). It is optimized for read-mostly scenarios, with consistent performance characteristics.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P1 review
</div>
