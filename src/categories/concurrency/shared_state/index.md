# Shared-State Concurrency

Channels are similar to single ownership, because once you transfer a value down a channel, you should no longer use that value. Shared memory concurrency is like multiple ownership: multiple threads can access the same memory location at the same time.

The Rust standard library provides smart pointer types, such as `Mutex<T>` and `Arc<T>`, that are safe to use in concurrent contexts.

{{#include index.incl.md}}

## Mutex

[![std][std-badge]][std]

Allow access to data from one thread at a time.

```rust,editable
{{#include ../../../../deps/tests/shared_state_mutex.rs}}
```

## Parking Lot

[![parking-lot][parking-lot-badge]][parking-lot]  [(crates.io)][parking-lot-crate]

[Parking Lot][parking-lot]⮳ provides implementations of `Mutex`, `RwLock`, `Condvar` and `Once` that are smaller, faster and more flexible than those in the Rust standard library. It also provides a `ReentrantMutex` type.

`std::sync::Mutex` works fine, but `Parking Lot` is faster.

```rust,editable,mdbook-runnable
{{#include ../../../../deps/tests/shared_state_parking_lot.rs}}
```

```rust,editable,mdbook-runnable
{{#include ../../../../deps/tests/shared_state_parking_lot2.rs}}
```

{{#include atomics.incl.md}}

{{#include ../../../refs/link-refs.md}}
