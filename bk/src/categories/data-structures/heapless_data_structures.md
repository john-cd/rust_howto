# Heapless Data Structures

{{#include heapless_data_structures.incl.md}}

## Work with Data Structures without Dynamic Memory Allocation {#heapless}

[![heapless][c~heapless~docs~badge]][c~heapless~docs] [![heapless~crates.io][c~heapless~crates.io~badge]][c~heapless~crates.io] [![heapless~repo][c~heapless~repo~badge]][c~heapless~repo] [![heapless~lib.rs][c~heapless~lib.rs~badge]][c~heapless~lib.rs]{{hi:heapless}}{{hi:No heap}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

The [`heapless`][c~heapless~docs]↗{{hi:heapless}} crate provides fixed-size data structures that don't require dynamic memory allocation. This means they are backed by static memory allocation (on the stack, in a static variable, or even in the heap, despite the name) and have _fixed_ capacities determined at compile time. They don't implicitly and unpredictably re-allocate at runtime.

This can be particularly useful for [[embedded]] systems or performance-critical applications, where dynamic memory allocation is not feasible or desirable: operations on [`heapless`][c~heapless~docs]↗{{hi:heapless}} data structures do not involve a memory allocator, reducing the risk of memory allocation failures. In addition, operations like `push` and `pop` are truly constant time, as there is no dynamic resizing involved.

[`heapless`][c~heapless~docs]↗{{hi:heapless}} includes:

- `heapless::pool::arc::Arc`, like [`std::sync::Arc`][c~std::sync::Arc~docs]↗{{hi:std::sync::Arc}} but backed by a lock-free memory pool rather than `#[global_allocator]`.
- `heapless::pool::boxed::Box`, like [`std::boxed::Box`][c~std::boxed::Box~docs]↗{{hi:std::boxed::Box}} but backed by a lock-free memory pool rather than `#[global_allocator]`.
- `heapless::binary_heap::BinaryHeap`, a priority queue.
- `heapless::IndexMap`, like [`IndexMap`][c~indexmap~docs]↗{{hi:indexmap}}.
- `heapless::IndexSet` and `FnvIndexSet`, like [`indexmap::set::IndexSet`][c~indexmap::set::IndexSet~docs]↗{{hi:indexmap::set::IndexSet}}, a hash set where the iteration order of the values is independent of their hash values.
- `heapless::LinearMap`, a fixed-capacity map / dictionary that performs lookups via linear search.
- `heapless::pool::object::Object`, an object managed by an object pool.
- `heapless::String`, a fixed capacity [`String`][c~std::string::String~docs]↗.
- `heapless::Vec`, a fixed capacity `Vec`.
- `heapless::mpmc::Q*`, a fixed-capacity multiple-producer multiple-consumer lock-free queues.
- `heapless::spsc::Queue`, a statically allocated single-producer single-consumer lock-free queue.

Keep in mind that choosing `heapless` is a trade-off: gain performance and determinism, but lose the flexibility of dynamic resizing. The following example demonstrates the basic usage of `heapless` collections:

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/heapless.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[binary_heaps | Binary Heaps]].
- [[hashmaps | Hash Maps]].
- [[heap_storage | Heap Storage]].
- [[other_maps | Other Maps]].
- [[stack_allocated_arrays | Stack-allocated Arrays]].
- [[strings | Strings]].
- [[vectors | Vectors]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
