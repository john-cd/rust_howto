# Vectors

{{#include vector.incl.md}}

## Store Homogeneous Data in a Contiguous, Growable Array: `Vec` {#vec}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}{{hi:std::vec::Vec}}{{hi:Vector}}

The vector type [`Vec<T>`]( )↗{{hi: }} is the go-to, general-purpose data structure, when you need a collection of elements that can change in size.

- It is a contiguous, growable, owned, heap-allocated array.
- It can only store values that are the same type.
- It has O(1) indexing, amortized O(1) [`push`]( )↗{{hi: }} (to the end) and O(1) `pop` (from the end).
- Many other data structures are built upon or interact with `Vec`.

The following example demonstrates common operations:

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/vector/vectors.rs:example}}
```

## `Vec` Memory Allocation {#memory-allocation}

`Vec` allocates memory on the heap (using the global allocator in `std::alloc`) to store its elements. `Vec` maintains a _capacity_ (how many elements it can hold without reallocating) and a _length_ (how many elements it currently holds).

When you push elements and exceed the current capacity, [`Vec`]( )↗{{hi: }} reallocates memory - usually by doubling the capacity. This amortized growth strategy ensures that pushing elements remains efficient over time. The old memory is deallocated, and the contents are copied to the new, larger allocation.

If allocation fails, Rust will abort the program using `alloc::handle_alloc_error`. For zero-sized types (like `()`), `Vec` avoids actual allocation.

Use the following suggestions to optimize memory usage:

```rust,editable
// If you know the size in advance, using `Vec::with_capacity(n)` can avoid multiple reallocations:
let mut v = Vec::with_capacity(100);
// Shrink after removing elements or if the vector has grown significantly beyond what you need:
v.shrink_to_fit();
// Cloning large vectors or their elements can be costly:
// - Prefer borrowing (`&T`) or moving (`T`) when possible.
// - Use slices when you don't need ownership:
fn process(_data: &[T]) { }
// Reuse vectors: Instead of creating new vectors in a loop, clear and reuse them to keep the capacity:
v.clear();
```

For fixed-size collections, use arrays. For small collections, using stack-allocated vectors may be more efficient by avoiding heap allocation altogether.

Read [Implementing Vec][book~rustonomicon~implementing-vec]↗ for more details about `Vec` memory management.

## Other Data Structures {#skip}

- [[btrees | B-trees]].
- [[binaryheap | Binary Heaps]].
- [[hashmaps | Hashmaps]].
- [[heapless | Heapless data structures]].
- [[maps | Other Maps]].
- [[slices | Slices]].
- [[stack_allocated_arrays | Stack Allocated Arrays]].
- [[stack_and_queue | Stacks and Queues]].
- [[strings | Strings]].

## Related Topics {#related-topics}

- [[algorithms | Algorithms]].
- [[encoding | Encoding]].
- [[rust-patterns | Rust Patterns]].
- [[sorting | Sorting]].
- [[typecasts | Typecasts]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
