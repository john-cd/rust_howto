# Vectors

{{#include vectors.incl.md}}

## Store Homogeneous Data in a Contiguous, Growable Array: `Vec` {#vec}

[![std][c-std-badge]][c-std]{{hi:std}}{{hi:Vec}}{{hi:Vectors}}

The vector type `Vec<T>` is the go-to, general-purpose data structure, when you need a collection of elements that can change in size.

- It is a contiguous, growable, owned, heap-allocated array.
- It can only store values that are the same type.
- It has O(1) indexing, amortized O(1) `push` (to the end) and O(1) `pop` (from the end).
- Many other data structures are built upon or interact with `Vec`.

The following example demonstrates common operations:

```rust,editable
{{#include ../../../crates/cats/data_structures/tests/vec/vectors.rs:example}}
```

## Related Data Structures {#skip}

- [[btrees | B-trees]].
- [[binaryheap | Binary Heaps]].
- [[hashmaps | Hashmaps]].
- [[heapless | Heapless data structures]].
- [[maps | Other Maps]].
- [[slices | Slices]].
- [[stack_allocated_arrays | Stack Allocated Arrays]].
- [[stack_and_queue | Stacks and Queues]].
- [[strings | Strings]].

## See Also

- [[algorithms | Algorithms]].
- [[encoding | Encoding]].
- [[rust-patterns | Rust Patterns]].
- [[sorting | Sorting]].
- [[typecasts | Typecasts]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[vectors: review](https://github.com/john-cd/rust_howto/issues/631)
</div>
