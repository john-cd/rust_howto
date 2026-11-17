<!-- TODO Reviewed ??-??-2026. -->

# Atomics

{{#include atomics.incl.md}}

From `bk/crates`, run `cargo test --locked -p concurrency --example atomics`.

## Standard Atomic Types {#atomics}

[![std][c~std~docs~badge]][c~std~docs] [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}{{hi:Atomics}}{{hi:std::sync::atomic}}

Atomic types{{hi:Atomic types}} in [`std::sync::atomic`][c~std::sync::atomic~docs]↗ provide primitive shared-memory communication between threads{{hi:Threads}}, and are the building blocks of other concurrent types.

The [`std::sync::atomic`][c~std::sync::atomic~docs]↗ module provides [`std::sync::atomic::AtomicBool`][c~std::sync::atomic::AtomicBool~docs]↗{{hi:std::sync::atomic::AtomicBool}}, [`std::sync::atomic::AtomicIsize`][c~std::sync::atomic::AtomicIsize~docs]↗{{hi:std::sync::atomic::AtomicIsize}}, [`std::sync::atomic::AtomicUsize`][c~std::sync::atomic::AtomicUsize~docs]↗{{hi:std::sync::atomic::AtomicUsize}}, [`std::sync::atomic::AtomicI8`][c~std::sync::atomic::AtomicI8~docs]↗{{hi:std::sync::atomic::AtomicI8}}, [`std::sync::atomic::AtomicU16`][c~std::sync::atomic::AtomicU16~docs]↗{{hi:std::sync::atomic::AtomicU16}}, etc.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/atomics/atomics.rs:example}}
```

To share ownership of an atomic variable across threads, put it in an [`std::sync::Arc`][c~std::sync::Arc~docs]↗{{hi:std::sync::Arc}} (an atomically-reference-counted shared pointer).

## AtomicCell with `crossbeam` {#crossbeam-atomics}

[![crossbeam-utils~website][c~crossbeam-utils~website~badge]][c~crossbeam-utils~website] {{#badges crossbeam-utils}} {{hi:Atomic}} {{hi:Cache}} {{hi:Scoped}} {{hi:Thread}} {{#categories algorithms=Algorithms,concurrency=Concurrency,data-structures="Data structures",no-std="No standard library"}}

`crossbeam-utils` provides miscellaneous tools for concurrent programming. It offers [`crossbeam_utils::atomic::AtomicCell`][c~crossbeam-utils::atomic::AtomicCell~docs]↗{{hi:crossbeam_utils::atomic::AtomicCell}}, a thread-safe mutable memory location. This type is equivalent to [`std::cell::Cell`][c~std::cell::Cell~docs]↗{{hi:std::cell::Cell}}, except it can also be shared among multiple threads. Operations on `AtomicCell`s use atomic instructions whenever possible, and synchronize using global locks otherwise.

```rust,ignore,noplayground
{{#include ../../../crates/cats/concurrency/examples/atomics/atomic_cell.rs:example}}
```

## Read-mostly Snapshots with `arc-swap` {#arc-swap}

{{#badges arc-swap data-structures="Data structures" memory-management="Memory management"}}{{hi:Arc}}{{hi:Atomic}}

The [`arc-swap::ArcSwap`][c~arc-swap::ArcSwap~docs]↗{{hi:arc-swap::ArcSwap}} type in [`arc-swap`][c~arc-swap~docs]↗{{hi:arc-swap}} stores an `Arc` that can be replaced atomically. Readers can keep using the snapshot they loaded while a writer publishes a replacement. It is designed for read-mostly shared state; use a mutex when callers need to mutate the value in place. This page describes the API but does not include a runnable `arc-swap` recipe.

## Build a Spinlock Using Atomics {#spinlock}

[![std][c~std~docs~badge]][c~std~docs] [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}{{hi:Spinlock}}{{hi:std::sync::atomic::AtomicBool}}

A spinlock{{hi:Spinlock}} is a simple synchronization primitive that uses busy-waiting (spinning) to acquire a lock. A spinlock continually retries acquisition. A [`std::sync::Mutex`][c~std::sync::Mutex~docs]↗{{hi:std::sync::Mutex}} can block a waiting thread under contention. Prefer a mutex for application code and treat this spinlock as an example of acquire and release ordering.

The following example implements a spinlock using [`std::sync::atomic::AtomicBool`][c~std::sync::atomic::AtomicBool~docs]↗{{hi:std::sync::atomic::AtomicBool}} with RAII-based locking via a guard type:

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/atomics/spinlock.rs:example}}
```

`Spinlock<T>` is `Sync` when `T: Send`, because access to its protected value goes through the lock. Its guard carries an exclusive-access marker, so a guard is `Send` when `T: Send` but is `Sync` only when `T: Sync`. This prevents two threads from sharing one guard to access a non-`Sync` value such as `Cell<T>` concurrently.

See [Model-Checking with Loom](loom.md) for a test that distinguishes acquire and release ordering from relaxed ordering.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
