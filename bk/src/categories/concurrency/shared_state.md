# Shared-State Concurrency

{{#include shared_state.incl.md}}

Channels{{hi:Channels}} are similar to single ownership, because once you transfer a value down a channel, you should no longer use that value. Shared memory concurrency{{hi:Shared memory concurrency}} is like multiple ownership: multiple threads can access the same memory location at the same time.

The Rust [standard library][p-standard-library] provides smart pointer types, such as `Mutex<T>`{{hi:Mutex}} and `Arc<T>`{{hi:Arc}}, that are safe to use in concurrent contexts.

## Maintain a Global Mutable State {#global-mutable-state}

[![lazy_static][c-lazy_static-badge]][c-lazy_static] [![lazy_static-crates.io][c-lazy_static-crates.io-badge]][c-lazy_static-crates.io] [![lazy_static-github][c-lazy_static-github-badge]][c-lazy_static-github] [![lazy_static-lib.rs][c-lazy_static-lib.rs-badge]][c-lazy_static-lib.rs]{{hi:lazy_static}}{{hi:Macro}}{{hi:Lazy}}{{hi:Static}} [![cat-memory-management][cat-memory-management-badge]][cat-memory-management]{{hi:Memory management}} [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}{{hi:Global mutable state}}

Declare global state using [`lazy static`][c-lazy_static]{{hi:lazy_static}}{{hi:Lazy static}}. [`lazy static`][c-lazy_static]{{hi:lazy_static}}⮳ creates a globally available `static ref` which requires a [`std::sync::Mutex`][c-std::sync::Mutex]{{hi:std::sync::Mutex}}⮳ to allow mutation (also see [`std::sync::RwLock`][c-std::sync::RwLock]{{hi:std::sync::RwLock}}⮳). The [`std::sync::Mutex`][c-std::sync::Mutex]{{hi:std::sync::Mutex}}⮳ wrap ensures the state cannot be simultaneously accessed by multiple threads, preventing race conditions. A [`std::sync::MutexGuard`][c-std::sync::MutexGuard]{{hi:std::sync::MutexGuard}}⮳ must be acquired to read or mutate the value stored in a [`std::sync::Mutex`][c-std::sync::Mutex]{{hi:std::sync::Mutex}}⮳.

```rust,editable
{{#include ../../../crates/cats/concurrency/tests/shared_state/global_mut_state.rs:example}}
```

## Mutexes {#mutex}

[![std][c-std-badge]][c-std] [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}{{hi:Mutex}}

Allow access to data from one thread at a time.

```rust,editable
{{#include ../../../crates/cats/concurrency/tests/shared_state/shared_state_mutex.rs:example}}
```

## `parking_lot` {#parking-lot}

[![parking_lot][c-parking_lot-badge]][c-parking_lot] [![parking_lot-crates.io][c-parking_lot-crates.io-badge]][c-parking_lot-crates.io] [![parking_lot-github][c-parking_lot-github-badge]][c-parking_lot-github] [![parking_lot-lib.rs][c-parking_lot-lib.rs-badge]][c-parking_lot-lib.rs]{{hi:parking_lot}}{{hi:Mutex}}{{hi:Thread}}{{hi:Rwlock}}{{hi:Condvar}}{{hi:Once}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

[`parking_lot`][c-parking_lot]⮳{{hi:parking_lot}} is a more compact and efficient implementation of the standard synchronization primitives.

[`parking_lot`][c-parking_lot]{{hi:parking_lot}}⮳ provides implementations of [`parking_lot::Mutex`][c-parking_lot::Mutex]{{hi:parking_lot::Mutex}}⮳, [`parking_lot::RwLock`][c-parking_lot::RwLock]{{hi:parking_lot::RwLock}}⮳, [`parking_lot::Condvar`][c-parking_lot::Condvar]{{hi:parking_lot::Condvar}}⮳ and [`parking_lot::Once`][c-parking_lot::Once]{{hi:parking_lot::Once}}⮳ that are smaller, faster and more flexible than those in the Rust standard library. It also provides a [`parking_lot::ReentrantMutex`][c-parking_lot::ReentrantMutex]{{hi:parking_lot::ReentrantMutex}}⮳ type.

[`std::sync::Mutex`][c-std::sync::Mutex]⮳{{hi:std::sync::Mutex}} {{hi:std::sync::Mutex}} works fine, but [`parking_lot`][c-parking_lot]⮳{{hi:parking_lot}} is faster.

```rust,editable
{{#include ../../../crates/cats/concurrency/tests/shared_state/shared_state_parking_lot.rs:example}}
```

```rust,editable
{{#include ../../../crates/cats/concurrency/tests/shared_state/shared_state_parking_lot2.rs:example}}
```

## Related Topics

- [[lazy_initialization | Lazy Initialization]].
- [[memory-management | Memory Management]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[shared_state: review; titles](https://github.com/john-cd/rust_howto/issues/266)
</div>
