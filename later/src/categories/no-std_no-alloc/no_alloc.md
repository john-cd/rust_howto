<!-- TODO Reviewed ??-??-2026 -->

# No-alloc

{{#include no_alloc.incl.md}}

No-allocation code avoids runtime heap allocation. This is useful when a target has no allocator or when memory use must have a predictable upper bound. `#![no_std]` alone does not disable heap allocation; see [[no-std | No `std`]] for the distinction.

Fixed-capacity containers reserve their storage as part of their type, so their capacity cannot grow at runtime. Treat a full container as an expected condition: use fallible operations and decide whether to reject, retry, or defer the item. Some convenience methods panic when full.

## Fixed-capacity collections

[`heapless`][c~heapless~docs] collections such as `heapless::Vec<T, N>` and `heapless::String<N>` store their data inline. [`arrayvec`][c~arrayvec~docs] offers `ArrayVec<T, N>` and `ArrayString<N>` with the same fixed-capacity idea. For strings, capacity is measured in bytes, not Unicode characters. Choose a capacity from the largest input the program is meant to accept, and use fallible methods such as `push` on `heapless::Vec` or `try_push` on `ArrayVec` when overflow must be handled.

## Format into a fixed buffer

Implementations of [`core::fmt::Write`][c~core::fmt::Write~docs] can receive `write!` output without creating an allocating `String`. A fixed-capacity string reports formatting failure when the output does not fit, so propagate or handle the returned `fmt::Result`. The example below formats into a `heapless::String`. [`ufmt`][c~ufmt~docs] is another option for targets where a smaller formatting implementation is useful; it supports a narrower set of formatting features.

## Pass messages through a bounded queue

[`heapless::spsc::Queue`][c~heapless::spsc~docs] is a fixed-capacity, lock-free single-producer, single-consumer queue. Split it into producer and consumer handles to pass messages between, for example, an interrupt and the main loop. Enqueueing into a full queue returns the item; dequeuing an empty queue returns `None`, so handle both cases. For `Queue<T, N>`, the usable capacity is `N - 1`. This queue requires atomic load/store support on the target or `heapless`'s `portable-atomic` feature.

## Reserve static storage and reuse objects

Use [`heapless::pool`][c~heapless::pool~docs] when a bounded set of objects must be shared and reused without a global allocator. The pool's storage is reserved up front, so code must handle the pool being exhausted. [`StaticCell`][c~static_cell~docs] provides a different pattern: reserve one value in static storage and initialize it at runtime, for example when constructing a peripheral or queue that cannot be initialized in a `const` context. Initialize it only once and handle an already-initialized cell where that can occur.

## Serialize into a caller-provided slice

[`postcard::to_slice`][c~postcard::to_slice~docs] serializes a `serde::Serialize` value into a mutable byte slice supplied by the caller. It returns the portion containing the encoded message, or an error if the buffer is too small. This keeps the output bounded and avoids allocating a temporary `Vec`; configure Serde without its default `std` feature for `no_std` use.

## Example {#noalloc}

The example fills a `heapless::Vec<u8, 8>`, handles the rejected ninth item, and formats the count into a `heapless::String<32>`. A failed push returns the item without changing the vector; `write!` propagates formatting failure. The snippet uses only `core` and `heapless` with default features disabled, so its storage operations do not require `std` or an allocator. The surrounding example binary runs on the host.

```rust,editable
{{#include ../../../crates/cats/no_std_no_alloc/examples/no_alloc/no_alloc.rs:example}}
```

From `later/crates`, run `cargo run -p no_std_no_alloc --example no_alloc`. The assertions demonstrate the full-vector behavior and expected formatted content.

## Related Topics {#related-topics .skip}

- [[no-std | No `std`]].
- [[embedded | Embedded Systems]].
- [[memory-management | Memory Management]].
- [[development_tools | Development Tools]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
