<!-- TODO Reviewed ??-??-2026 -->

# Fixed-capacity and Stack-allocated Data Structures

{{#include stack-allocated.incl.md}}

An array or fixed-capacity collection stores its elements inline with the collection value. A local value is usually placed on the stack, but the same value can also be stored in static memory or inside another allocation. The important distinction is whether the collection asks an allocator for more space as it grows.

| Crate | Capacity and growth | When the collection is full |
|-------|---------------------|-----------------------------|
| [`arrayvec`][c~arrayvec~docs]↗ | Fixed capacity; no allocation when growing | `push` panics; `try_push` returns the rejected item |
| [`heapless`][c~heapless~docs]↗ | Fixed capacity; does not use a heap allocator | Operations such as `push` return an error with the rejected item |
| [`smallvec`][c~smallvec~docs]↗ | Stores a configured number of elements inline, then can allocate and grow | Grows like a vector after spilling to the heap |
| [`tinyvec`][c~tinyvec~docs]↗ | With `alloc`, `TinyVec` starts inline and can move to heap storage; without it, `ArrayVec` has fixed capacity | `TinyVec` grows when it spills; `ArrayVec` panics if full |

Fixed-capacity collections are useful when a maximum size is known or dynamic allocation is unavailable. Choose an inline capacity with care: the collection value occupies space for that capacity even when it is empty, and large local values can use substantial stack space. `SmallVec` and `TinyVec` trade a larger inline value for avoiding allocations in common small cases. These layouts do not guarantee faster code; measure when performance matters.

## Store Fixed-size Vectors and Strings on the Stack with `arrayvec` {#arrayvec}

[![arrayvec][c~arrayvec~docs~badge]][c~arrayvec~docs] [![arrayvec~crates.io][c~arrayvec~crates.io~badge]][c~arrayvec~crates.io] [![arrayvec~repo][c~arrayvec~repo~badge]][c~arrayvec~repo] [![arrayvec~lib.rs][c~arrayvec~lib.rs~badge]][c~arrayvec~lib.rs]{{hi:arrayvec}}{{hi:Array}}{{hi:Data-structure}}{{hi:No_std}}{{hi:Stack}}{{hi:Vector}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`arrayvec`][c~arrayvec~docs]↗{{hi:arrayvec}} provides the types `ArrayVec` and `ArrayString`, which are stack-allocated, fixed-capacity array-backed vector and string types.

`ArrayVec` is a vector-like collection with a fixed capacity that is determined at compile time. `ArrayVec` allocates its storage on the stack rather than on the heap, which can lead to better performance. It offers a simple API but also dereferences to a `slice`, so that the full `slice` API is available:

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/stack_allocated/arrayvec.rs:example}}
```

## Store Small Vectors on the Stack with Fallback to the Heap {#smallvec-tinyvec}

[![smallvec][c~smallvec~docs~badge]][c~smallvec~docs] [![smallvec~crates.io][c~smallvec~crates.io~badge]][c~smallvec~crates.io] [![smallvec~repo][c~smallvec~repo~badge]][c~smallvec~repo] [![smallvec~lib.rs][c~smallvec~lib.rs~badge]][c~smallvec~lib.rs]{{hi:smallvec}}{{hi:No_std}}{{hi:Small}}{{hi:Stack}}{{hi:Vec}}{{hi:Vector}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

[![tinyvec][c~tinyvec~docs~badge]][c~tinyvec~docs] [![tinyvec~crates.io][c~tinyvec~crates.io~badge]][c~tinyvec~crates.io] [![tinyvec~repo][c~tinyvec~repo~badge]][c~tinyvec~repo] [![tinyvec~lib.rs][c~tinyvec~lib.rs~badge]][c~tinyvec~lib.rs]{{hi:tinyvec}}{{hi:No-std}}{{hi:No_std}}{{hi:Vec}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

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

[![heapless][c~heapless~docs~badge]][c~heapless~docs] [![heapless~crates.io][c~heapless~crates.io~badge]][c~heapless~crates.io] [![heapless~repo][c~heapless~repo~badge]][c~heapless~repo] [![heapless~lib.rs][c~heapless~lib.rs~badge]][c~heapless~lib.rs]{{hi:heapless}}{{hi:No heap}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

The [`heapless`][c~heapless~docs]↗{{hi:heapless}} crate provides fixed-size data structures that don't require dynamic memory allocation. This means they are backed by _static memory allocation_ (on the stack, in a static variable, or even in the heap, despite the name) and have _fixed_ capacities determined at compile time. They don't implicitly and unpredictably re-allocate at runtime.

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

<div class="hidden">
[accuracy review: data structures](https://github.com/john-cd/rust_howto/issues/1892)
</div>
