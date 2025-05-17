# Vectors

{{#include vectors.incl.md}}

## Store Homogenous Data in a Contiguous, Growable Array: `Vec` {#vec}

[![std][c-std-badge]][c-std]{{hi:std}}{{hi:Vec}}{{hi:Vectors}}

The Vector type (`Vec`)  is a contiguous, growable, heap-allocated array. `Vec<T>` can only store values that are the same type.
Vectors have O(1) indexing, amortized O(1) `push` (to the end) and O(1) `pop` (from the end).

The following example demonstrates common operations with `Vec`:

```rust,editable
{{#include ../../crates/standard_library/tests/vec/vectors.rs:example}}
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
[vectors: review; move to data structure??](https://github.com/john-cd/rust_howto/issues/631)
</div>
