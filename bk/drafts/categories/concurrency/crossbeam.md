<!-- TODO Reviewed ??-??-2026 -->

# Crossbeam Channels and Concurrent Utilities

{{#include crossbeam.incl.md}}

Crossbeam is a family of crates for concurrent programming. `crossbeam-channel` adds bounded multi-producer, multi-consumer channels and `select!`; `crossbeam-queue` provides concurrent queues, while `crossbeam-utils` includes `AtomicCell`. The queue and atomic recipes live in [Concurrent Data Structures](concurrent_data_structures.md) and [Atomics](atomics.md).

Use `std::thread::scope` for scoped threads on current Rust; it replaces the thread-scoping recipe once provided by Crossbeam. Use the [Message Passing](message_passing.md) chapter for general channel patterns.

## Select Across Several Channels {#select-channels}

[![crossbeam-channel][c~crossbeam-channel~docs~badge]][c~crossbeam-channel~docs] [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]

`select!` waits for one of several channel operations to become ready. If more than one is ready, Crossbeam selects one without promising a fixed order.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/crossbeam/crossbeam_select.rs:example}}
```

## Distribute Work with a Bounded MPMC Pipeline {#parallel-pipeline}

[![crossbeam-channel][c~crossbeam-channel~docs~badge]][c~crossbeam-channel~docs] [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]

A bounded channel limits queued work; cloned receivers let multiple workers compete for each message, so every item is handled by one worker.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/crossbeam/crossbeam_complex.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Explicit Threads](explicit_threads.md) for `std::thread::scope`
- [Message Passing](message_passing.md) for channel fundamentals
- [Concurrent queues][ex~concurrency~crossbeam-queue]
- [AtomicCell][ex~concurrency~crossbeam-atomics]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
