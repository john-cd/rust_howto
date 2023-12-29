# Shared-State Concurrency

Channels are similar to single ownership, because once you transfer a value down a channel, you should no longer use that value. Shared memory concurrency is like multiple ownership: multiple threads can access the same memory location at the same time.

The Rust standard library provides smart pointer types, such as `Mutex<T>` and `Arc<T>`, that are safe to use in concurrent contexts.

## Mutex

Allow access to data from one thread at a time.

```rust,editable
{{#include ../../deps/examples/shared_state_mutex.rs}}
```

## Parking Lot

[Parking Lot]( https://crates.io/crates/parking_lot ) provides implementations of `Mutex`, `RwLock`, `Condvar` and `Once` that are smaller, faster and more flexible than those in the Rust standard library. It also provides a `ReentrantMutex` type.

`std::sync::Mutex` works fine, but Parking Lot is faster.

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/shared_state_parking_lot.rs}}
```

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/shared_state_parking_lot2.rs}}
```

## Atomics

Atomic types in [std::sync::atomic]( https://doc.rust-lang.org/std/sync/atomic/index.html ) provide primitive shared-memory communication between threads, and are the building blocks of other concurrent types. It defines atomic versions of a select number of primitive types, including `AtomicBool`, `AtomicIsize`, `AtomicUsize`, `AtomicI8`, `AtomicU16`, etc.

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/shared_state_atomics.rs}}
```

The most common way to share an atomic variable is to put it into an `Arc` (an atomically-reference-counted shared pointer).

[crossbeam](https://docs.rs/crossbeam/latest/crossbeam/) also offers `AtomicCell`, a thread-safe mutable memory location. This type is equivalent to `Cell`, except it can also be shared among multiple threads.

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/shared_state_atomics.rs}}
```
