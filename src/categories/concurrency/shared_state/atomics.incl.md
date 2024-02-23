## Atomics

[![std][std-badge]][std]  [![crossbeam][crossbeam-badge]][crossbeam]

Atomic types in [`std::sync::atomic`][std::sync::atomic]⮳ provide primitive shared-memory communication between threads, and are the building blocks of other concurrent types. It defines atomic versions of a select number of primitive types, including `AtomicBool`, `AtomicIsize`, `AtomicUsize`, `AtomicI8`, `AtomicU16`, etc.

```rust,editable,mdbook-runnable
{{#include ../../../../deps/tests/shared_state_atomics.rs}}
```

The most common way to share an atomic variable is to put it into an `Arc` (an atomically-reference-counted shared pointer).

[`crossbeam`][crossbeam]⮳ also offers `AtomicCell`, a thread-safe mutable memory location. This type is equivalent to `Cell`, except it can also be shared among multiple threads.

```rust,editable,mdbook-runnable
{{#include ../../../../deps/tests/shared_state_crossbeam.rs}}
```

{{#include ../../../refs/link-refs.md}}
