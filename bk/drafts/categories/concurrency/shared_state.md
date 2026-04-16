# Shared-State Concurrency

{{#include shared_state.incl.md}}

Channels{{hi:Channels}} are similar to single ownership, because once a value is transferred down a channel, it can no longer be used. Shared memory concurrency{{hi:Shared memory concurrency}} is like multiple ownership: multiple threads can access the same memory location at the same time.

The Rust [standard library][p~standard-library] provides smart pointer types, such as `Mutex<T>`{{hi:Mutex}} and `Arc<T>`{{hi:Arc}}, that are safe to use in concurrent contexts.

## Shared ownership with `Arc` {#arc}

The type `Arc<T>` provides shared ownership of a value of type `T`, allocated in the heap. Invoking clone on `Arc` produces a new `Arc` instance, which points to the _same allocation_ on the heap as the source `Arc`, while increasing a reference count. When the last `Arc` pointer to a given allocation is destroyed, the value stored in that allocation (often referred to as "inner value") is also dropped.

Shared references in Rust disallow mutation by default, and `Arc` is no exception: you _cannot generally obtain a mutable reference to something inside an `Arc`_. If you do need to mutate through an `Arc`, you have several options:

- Use interior mutability with synchronization primitives like `Mutex`, `RwLock`, or one of the `Atomic` types.
- Use clone-on-write semantics with `Arc::make_mut` which provides efficient mutation without requiring interior mutability. This approach clones the data only when needed (when there are multiple references) and can be more efficient when mutations are infrequent.
- Use `Arc::get_mut` when you know your `Arc` is not shared (has a reference count of 1), which provides direct mutable access to the inner value without any cloning.

## Clone-on-write with `Arc::make_mut` {#arc_make_mut}

Use clone-on-write semantics with `Arc::make_mut` which provides efficient mutation without requiring interior mutability. This approach clones the data only when needed (when there are multiple references) and can be more efficient when mutations are infrequent.

[`Rc::make_mut`][c~std::rc::Rc::make_mut~docs]↗{{hi:std::rc::Rc::make_mut}} and [`Arc::make_mut`][c~std::sync::Arc::make_mut~docs]↗{{hi:std::rc::Arc::make_mut}} can provide clone-on-write functionality.

```rust,editable
{{#include ../../crates/cats/concurrency/examples/shared_state/arc_make_mut.rs:example}}
```

## Maintain a Global Mutable State {#global-mutable-state}

[![lazy_static][c~lazy_static~docs~badge]][c~lazy_static~docs] [![lazy_static~crates.io][c~lazy_static~crates.io~badge]][c~lazy_static~crates.io] [![lazy_static~repo][c~lazy_static~repo~badge]][c~lazy_static~repo] [![lazy_static~lib.rs][c~lazy_static~lib.rs~badge]][c~lazy_static~lib.rs]{{hi:lazy_static}}{{hi:Macro}}{{hi:Lazy}}{{hi:Static}} [![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}} [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}{{hi:Global mutable state}}

Declare global state using [`lazy static`][c~lazy_static~docs]↗{{hi:lazy_static}}{{hi:Lazy static}}. [`lazy static`][c~lazy_static~docs]↗{{hi:lazy_static}} creates a globally available `static ref` which requires a [`std::sync::Mutex`][c~std::sync::Mutex~docs]↗{{hi:std::sync::Mutex}} to allow mutation (also see [`std::sync::RwLock`][c~std::sync::RwLock~docs]↗{{hi:std::sync::RwLock}}). The [`std::sync::Mutex`][c~std::sync::Mutex~docs]↗{{hi:std::sync::Mutex}} wrap ensures the state cannot be simultaneously accessed by multiple threads, preventing race conditions. A [`std::sync::MutexGuard`][c~std::sync::MutexGuard~docs]↗{{hi:std::sync::MutexGuard}} must be acquired to read or mutate the value stored in a [`std::sync::Mutex`][c~std::sync::Mutex~docs]↗{{hi:std::sync::Mutex}}.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/shared_state/global_mut_state.rs:example}}
```

## Mutexes {#mutex}

[![std][c~std~docs~badge]][c~std~docs] [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}{{hi:Mutex}}

Allow access to data from one thread at a time.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/shared_state/shared_state_mutex.rs:example}}
```

## Using `Arc` and `Mutex` for Safe Concurrent Access to Shared Data {#arc-mutex}

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/shared_state/send_sync.rs:example}}
```

## `parking_lot` {#parking-lot}

[![parking_lot][c~parking_lot~docs~badge]][c~parking_lot~docs] [![parking_lot~crates.io][c~parking_lot~crates.io~badge]][c~parking_lot~crates.io] [![parking_lot~repo][c~parking_lot~repo~badge]][c~parking_lot~repo] [![parking_lot~lib.rs][c~parking_lot~lib.rs~badge]][c~parking_lot~lib.rs]{{hi:parking_lot}}{{hi:Mutex}}{{hi:Thread}}{{hi:Rwlock}}{{hi:Condvar}}{{hi:Once}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

[`parking_lot`][c~parking_lot~docs]↗{{hi:parking_lot}} is a more compact and efficient implementation of the standard synchronization primitives.

[`parking_lot`][c~parking_lot~docs]↗{{hi:parking_lot}} provides implementations of [`parking_lot::Mutex`][c~parking_lot::Mutex~docs]↗{{hi:parking_lot::Mutex}}, [`parking_lot::RwLock`][c~parking_lot::RwLock~docs]↗{{hi:parking_lot::RwLock}}, [`parking_lot::Condvar`][c~parking_lot::Condvar~docs]↗{{hi:parking_lot::Condvar}} and [`parking_lot::Once`][c~parking_lot::Once~docs]↗{{hi:parking_lot::Once}} that are smaller, faster and more flexible than those in the Rust standard library. It also provides a [`parking_lot::ReentrantMutex`][c~parking_lot::ReentrantMutex~docs]↗{{hi:parking_lot::ReentrantMutex}} type.

[`std::sync::Mutex`][c~std::sync::Mutex~docs]↗{{hi:std::sync::Mutex}} {{hi:std::sync::Mutex}} works fine, but [`parking_lot`][c~parking_lot~docs]↗{{hi:parking_lot}} is faster.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/shared_state/shared_state_parking_lot.rs:example}}
```

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/shared_state/shared_state_parking_lot2.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[lazy_initialization | Lazy Initialization]].
- [[memory-management | Memory Management]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[shared_state: reorganize; section for Arc; for Mutex / Rwlock; for joint use; for make_mut](https://github.com/john-cd/rust_howto/issues/266)
</div>
