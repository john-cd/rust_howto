<!-- TODO Reviewed ??-??-2026 -->

# Shared-State Concurrency

{{#include shared_state.incl.md}}

With message passing, a sender gives up ownership of a value. Shared memory lets several threads access the same allocation, so mutation needs synchronization. Prefer a channel when it gives the data one clear owner; use shared state when threads must coordinate around one value.

The Rust [standard library][p~standard-library] provides `Arc<T>`{{hi:Arc}} for shared ownership and `Mutex<T>`{{hi:Mutex}}, `RwLock<T>`{{hi:RwLock}}, and atomic types for synchronized access. A standard-library mutex is a sound default; consider alternatives for a concrete API or measured workload need.

## Share Immutable Ownership with `Arc` {#arc}

The type `Arc<T>` provides shared ownership of a value of type `T`, allocated in the heap. Invoking clone on `Arc` produces a new `Arc` instance, which points to the _same allocation_ on the heap as the source `Arc`, while increasing a reference count. When the last `Arc` pointer to a given allocation is destroyed, the value stored in that allocation (often referred to as "inner value") is also dropped.

Shared references in Rust disallow mutation by default, and `Arc` is no exception: you _cannot generally obtain a mutable reference to something inside an `Arc`_. If you do need to mutate through an `Arc`, you have several options:

- Use interior mutability with synchronization primitives like `Mutex`, `RwLock`, or one of the `Atomic` types.
- Use clone-on-write semantics with `Arc::make_mut` which provides efficient mutation without requiring interior mutability. This approach clones the data only when needed (when there are multiple references) and can be more efficient when mutations are infrequent.
- Use `Arc::get_mut` when you know your `Arc` is not shared (has a reference count of 1), which provides direct mutable access to the inner value without any cloning.

## Update Shared Values with `Arc::make_mut` {#arc_make_mut}

Use `Arc::make_mut` when updates are infrequent and readers can keep an older snapshot. It mutates in place when the `Arc` is uniquely owned and clones the inner value when other strong references exist.

[`Rc::make_mut`][c~std::rc::Rc::make_mut~docs]↗{{hi:std::rc::Rc::make_mut}} and [`Arc::make_mut`][c~std::sync::Arc::make_mut~docs]↗{{hi:std::rc::Arc::make_mut}} can provide clone-on-write functionality.

```rust,editable
{{#include ../../crates/cats/concurrency/examples/shared_state/arc_make_mut.rs:example}}
```

## Maintain Global Mutable State {#global-mutable-state}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}} [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}{{hi:Global mutable state}}

A lazy initializer only controls construction; the `Mutex` controls access to the mutable value. This example uses `LazyLock<Mutex<T>>` so the global mutex is created on first use, then locks it around each read or write. Prefer avoiding global mutable state when it can be passed explicitly.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/shared_state/global_mut_state.rs:example}}
```

## Use a Mutex for Exclusive Mutation {#mutex}

[![std][c~std~docs~badge]][c~std~docs] [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}{{hi:Mutex}}

Use a mutex when one thread at a time may read or mutate a value. Keep the critical section short and avoid waiting on unrelated work while holding the guard.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/shared_state/shared_state_mutex.rs:example}}
```

## Share Mutable Data with `Arc<Mutex<T>>` {#arc-mutex}

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/shared_state/send_sync.rs:example}}
```

## Use `parking_lot` Primitives When They Fit {#parking-lot}

[![parking_lot][c~parking_lot~docs~badge]][c~parking_lot~docs] [![parking_lot~crates.io][c~parking_lot~crates.io~badge]][c~parking_lot~crates.io] [![parking_lot~repo][c~parking_lot~repo~badge]][c~parking_lot~repo] [![parking_lot~lib.rs][c~parking_lot~lib.rs~badge]][c~parking_lot~lib.rs]{{hi:parking_lot}}{{hi:Mutex}}{{hi:Thread}}{{hi:Rwlock}}{{hi:Condvar}}{{hi:Once}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

[`parking_lot`][c~parking_lot~docs]↗{{hi:parking_lot}} provides mutexes, read-write locks, condition variables, and one-time initialization with a different API and implementation tradeoffs. The standard library primitives work well for most uses; benchmark before choosing a lock for performance reasons.

[`parking_lot`][c~parking_lot~docs]↗{{hi:parking_lot}} provides implementations of [`parking_lot::Mutex`][c~parking_lot::Mutex~docs]↗{{hi:parking_lot::Mutex}}, [`parking_lot::RwLock`][c~parking_lot::RwLock~docs]↗{{hi:parking_lot::RwLock}}, [`parking_lot::Condvar`][c~parking_lot::Condvar~docs]↗{{hi:parking_lot::Condvar}} and [`parking_lot::Once`][c~parking_lot::Once~docs]↗{{hi:parking_lot::Once}} that are smaller, faster and more flexible than those in the Rust standard library. It also provides a [`parking_lot::ReentrantMutex`][c~parking_lot::ReentrantMutex~docs]↗{{hi:parking_lot::ReentrantMutex}} type.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/shared_state/shared_state_parking_lot.rs:example}}
```

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/shared_state/shared_state_parking_lot2.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Lazy initialization][p~lazy-initialization], including `std::sync::LazyLock`.
- [Message passing](message_passing.md) when ownership can move through a channel.
- [Concurrent data structures](concurrent_data_structures.md) for maps, queues, and atomic shared pointers.
- [Data parallelism][p~data-parallelism] for collection-oriented parallel work.
- [Model-checking with Loom][ex~concurrency~loom] to explore the interleavings of a small shared-state protocol.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
