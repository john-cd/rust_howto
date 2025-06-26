# Atomics

{{#include atomics.incl.md}}

## Standard Atomic Types {#atomics}

[![std][c~std~docs~badge]][c~std~docs] [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}{{hi:Atomics}}

Atomic types{{hi:Atomic types}} in [`std::sync::atomic`][c~std::sync::atomic~docs]{{hi:std::sync::atomic}}⮳ provide primitive shared-memory communication between threads{{hi:Threads}}, and are the building blocks of other concurrent types.

The `atomic` module provides [`std::sync::atomic::AtomicBool`][c~std::sync::atomic::AtomicBool~docs]{{hi:std::sync::atomic::AtomicBool}}⮳, [`std::sync::atomic::AtomicIsize`][c~std::sync::atomic::AtomicIsize~docs]{{hi:std::sync::atomic::AtomicIsize}}⮳, [`std::sync::atomic::AtomicUsize`][c~std::sync::atomic::AtomicUsize~docs]{{hi:std::sync::atomic::AtomicUsize}}⮳, [`std::sync::atomic::AtomicI8`][c~std::sync::atomic::AtomicI8~docs]{{hi:std::sync::atomic::AtomicI8}}⮳, [`std::sync::atomic::AtomicU16`][c~std::sync::atomic::AtomicU16~docs]{{hi:std::sync::atomic::AtomicU16}}⮳, etc.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/atomics/atomics.rs:example}}
```

The most common way to share an atomic variable is to put it into an [`std::sync::Arc`][c~std::sync::Arc~docs]{{hi:std::sync::Arc}}⮳ (an atomically-reference-counted shared pointer).

## AtomicCell with `crossbeam` {#crossbeam-atomics}

[![crossbeam-utils~website][c~crossbeam_utils~website~badge]][c~crossbeam_utils~website] [![crossbeam-utils][c~crossbeam_utils~docs~badge]][c~crossbeam_utils~docs] [![crossbeam-utils~crates.io][c~crossbeam_utils~crates.io~badge]][c~crossbeam_utils~crates.io] [![crossbeam-utils~github][c~crossbeam_utils~github~badge]][c~crossbeam_utils~github] [![crossbeam-utils~lib.rs][c~crossbeam_utils~lib.rs~badge]][c~crossbeam_utils~lib.rs]{{hi:crossbeam-utils}}{{hi:Atomic}}{{hi:Cache}}{{hi:Scoped}}{{hi:Thread}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

`crossbeam_utils`provides miscellaneous tools for concurrent programming. It offers [`crossbeam::atomic::AtomicCell`][c~crossbeam::atomic::AtomicCell~docs]{{hi:crossbeam::atomic::AtomicCell}}⮳, a thread-safe mutable memory location. This type is equivalent to [`std::cell::Cell`][c~std::cell::Cell~docs]{{hi:std::cell::Cell}}⮳, except it can also be shared among multiple threads. Operations on AtomicCells use atomic instructions whenever possible, and synchronize using global locks otherwise.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/atomics/atomic_cell.rs:example}}
```

## `arc-swap` {#arc-swap}

[![arc-swap][c~arc_swap~docs~badge]][c~arc_swap~docs] [![arc-swap~crates.io][c~arc_swap~crates.io~badge]][c~arc_swap~crates.io] [![arc-swap~github][c~arc_swap~github~badge]][c~arc_swap~github] [![arc-swap~lib.rs][c~arc_swap~lib.rs~badge]][c~arc_swap~lib.rs]{{hi:arc-swap}}{{hi:Arc}}{{hi:Atomic}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}}

The `ArcSwap` type in [`arc-swap`][c~arc_swap~docs]⮳{{hi:arc-swap}} is a container for an `Arc` that can be changed atomically. Semantically, it is similar to `Atomic<Arc<T>>` (if there was such a thing) or `RwLock<Arc<T>>` (but without the need for the locking). It is optimized for read-mostly scenarios, with consistent performance characteristics.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[fix NOW](https://github.com/john-cd/rust_howto/issues/1342)

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/atomics/spinlock.rs:example}}
```

</div>
