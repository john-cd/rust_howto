# Atomics

{{#include atomics.incl.md}}

## Standard Atomic Types {#atomics}

[![std][c-std-badge]][c-std] [![crossbeam][c-crossbeam-badge]][c-crossbeam]{{hi:crossbeam}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}{{hi:Atomics}}

Atomic types{{hi:Atomic types}} in [`std::sync::atomic`][c-std::sync::atomic]{{hi:std::sync::atomic}}⮳ provide primitive shared-memory communication between threads{{hi:Threads}}, and are the building blocks of other concurrent types. It defines atomic versions of a select number of primitive types, including [`std::sync::atomic::AtomicBool`][c-std::sync::atomic::AtomicBool]{{hi:std::sync::atomic::AtomicBool}}⮳, [`std::sync::atomic::AtomicIsize`][c-std::sync::atomic::AtomicIsize]{{hi:std::sync::atomic::AtomicIsize}}⮳, [`std::sync::atomic::AtomicUsize`][c-std::sync::atomic::AtomicUsize]{{hi:std::sync::atomic::AtomicUsize}}⮳, [`std::sync::atomic::AtomicI8`][c-std::sync::atomic::AtomicI8]{{hi:std::sync::atomic::AtomicI8}}⮳, [`std::sync::atomic::AtomicU16`][c-std::sync::atomic::AtomicU16]{{hi:std::sync::atomic::AtomicU16}}⮳, etc.

```rust,editable
{{#include ../../../crates/cats/concurrency/tests/shared_state/shared_state_atomics.rs:example}}
```

The most common way to share an atomic variable is to put it into an [`std::sync::Arc`][c-std::sync::Arc]{{hi:std::sync::Arc}}⮳ (an atomically-reference-counted shared pointer).

[`crossbeam`][c-crossbeam]{{hi:crossbeam}}⮳ also offers [`crossbeam::atomic::AtomicCell`][c-crossbeam::atomic::AtomicCell]{{hi:crossbeam::atomic::AtomicCell}}⮳, a thread-safe mutable memory location. This type is equivalent to [`std::cell::Cell`][c-std::cell::Cell]{{hi:std::cell::Cell}}⮳, except it can also be shared among multiple threads.

```rust,editable
{{#include ../../../crates/cats/concurrency/tests/shared_state/shared_state_crossbeam.rs:example}}
```

## Atomic Operations {#skip}

[`std::sync::atomic`][c-std::sync::atomic]⮳{{hi:std::sync::atomic}} provides atomic types for safe concurrent access to data. Essential for multi-threaded programming.

## `arc-swap` {#arc-swap}

[![arc-swap][c-arc_swap-badge]][c-arc_swap] [![arc-swap-crates.io][c-arc_swap-crates.io-badge]][c-arc_swap-crates.io] [![arc-swap-github][c-arc_swap-github-badge]][c-arc_swap-github] [![arc-swap-lib.rs][c-arc_swap-lib.rs-badge]][c-arc_swap-lib.rs]{{hi:arc-swap}}{{hi:Arc}}{{hi:Atomic}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}} [![cat-memory-management][cat-memory-management-badge]][cat-memory-management]{{hi:Memory management}}

The `ArcSwap` type in [`arc-swap`][c-arc_swap]⮳{{hi:arc-swap}} is a container for an `Arc` that can be changed atomically. Semantically, it is similar to `Atomic<Arc<T>>` (if there was such a thing) or `RwLock<Arc<T>>` (but without the need for the locking). It is optimized for read-mostly scenarios, with consistent performance characteristics.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[fix NOW](https://github.com/john-cd/rust_howto/issues/1342)
</div>
