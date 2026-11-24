<!-- TODO Reviewed ??-??-2026. -->

# Fixed-capacity and Stack-allocated Data Structures

{{#include stack-allocated.incl.md}}

An array or fixed-capacity collection stores its elements inline with the collection value. A local value is usually placed on the stack, but the same value can also be stored in static memory or inside another allocation. The important distinction is whether the collection asks an allocator for more space as it grows.

| Crate | Capacity and growth | When the collection is full |
| --- | --- | --- |
| [`arrayvec`][c~arrayvec~docs]↗ | Fixed capacity; no allocation when growing | `push` panics; `try_push` returns the rejected item |
| [`heapless`][c~heapless~docs]↗ | Fixed capacity; does not use a heap allocator | Operations such as `push` return an error with the rejected item |
| [`smallvec`][c~smallvec~docs]↗ | Stores a configured number of elements inline, then can allocate and grow | Grows like a vector after spilling to the heap |
| [`tinyvec`][c~tinyvec~docs]↗ | With `alloc`, `TinyVec` starts inline and can move to heap storage; without it, `ArrayVec` has fixed capacity | `TinyVec` grows when it spills; `ArrayVec` panics if full |

Fixed-capacity collections are useful when a maximum size is known or dynamic allocation is unavailable. Choose an inline capacity with care: the collection value occupies space for that capacity even when it is empty, and large local values can use substantial stack space. `SmallVec` and `TinyVec` trade a larger inline value for avoiding allocations in common small cases. These layouts do not guarantee faster code; measure when performance matters.

## Store Fixed-size Vectors and Strings on the Stack with `arrayvec` {#arrayvec}

{{#badges arrayvec data-structures="Data structures" no-std="No standard library"}}{{hi:Array}}{{hi:Data-structure}}{{hi:No_std}}{{hi:Stack}}{{hi:Vector}}

[`arrayvec`][c~arrayvec~docs]↗{{hi:arrayvec}} provides the types `ArrayVec` and `ArrayString`, which store vector elements and string bytes inline, up to a fixed capacity.

`ArrayVec` is a vector-like collection with a fixed capacity determined at compile time. Its elements are stored inline in the array-backed value, so it does not allocate when it grows; this does not by itself guarantee better performance. It offers a simple API and dereferences to a `slice`, so the full `slice` API is available:

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/stack_allocated/arrayvec.rs:example}}
```

## Store Small Vectors on the Stack with Fallback to the Heap {#smallvec-tinyvec}

{{#badges smallvec data-structures="Data structures"}}{{hi:No_std}}{{hi:Small}}{{hi:Stack}}{{hi:Vec}}{{hi:Vector}}

{{#badges tinyvec no-std="No standard library" data-structures="Data structures"}}{{hi:No-std}}{{hi:No_std}}{{hi:Vec}}

This example demonstrates the usage of the `SmallVec` data structure from the [`smallvec`][c~smallvec~docs]↗{{hi:smallvec}} crate. `SmallVec` is a vector-like data structure that stores elements inline when the number of elements is small, and switches to heap allocation when the number of elements exceeds its inline capacity:

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/stack_allocated/smallvec.rs:example}}
```

[`tinyvec`][c~tinyvec~docs]↗{{hi:tinyvec}} is similar to [`smallvec`][c~smallvec~docs]↗{{hi:smallvec}} but with a smaller feature set and no dependencies. It is written in 100% safe Rust code. However, `tinyvec` requires items to implement the [`Default`][c~std::default::Default~docs]↗{{hi:std::default::Default}} trait.

The following demonstrates the usage of `TinyVec`:

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/stack_allocated/tinyvec.rs:example}}
```

## Work with Data Structures without Dynamic Memory Allocation {#heapless}

{{#badges heapless data-structures="Data structures" no-std="No standard library"}}{{hi:No heap}}

The [`heapless`][c~heapless~docs]↗{{hi:heapless}} collections in this example store their elements inline. Their capacities are fixed at compile time.

For an [embedded](../embedded/index.md) application, a fixed capacity gives the collection a predictable storage requirement. Pushing onto a full `heapless::Vec` returns the rejected element, so the caller can decide how to handle it.

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
- `heapless::mpmc::Queue`, a fixed-capacity multiple-producer multiple-consumer queue (the `Q*` aliases were removed in heapless 0.9; this module is deprecated in that release).
- `heapless::spsc::Queue`, a statically allocated single-producer single-consumer lock-free queue.

The example shows how to create and use fixed-capacity collections:

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/stack_allocated/heapless.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[binary_heaps | Binary Heaps]].
- [[hashmaps | Hash Maps]].
- [[heap_storage | Heap Storage]].
- [[other_maps | Other Maps]].
- [[slices | Slices]].
- [[strings | Strings]].
- [[vectors | Vectors]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
