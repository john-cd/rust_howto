# Linked Lists

{{#include linkedlist.incl.md}}

## Store Data in a Linked List {#linkedlist}

[![std][c~std~docs~badge]][c~std~docs] [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]

`LinkedList` represents a doubly linked list, where each element contains a reference to both the next and the previous elements. This allows for efficient insertions and deletions (in constant time) from both ends of the list. You can also iterate over the elements of the list in both forward and backward directions. However, randomly accessing elements is slower (in O(n) time) compared to arrays, due to the linear nature of the structure.

Except for a few niche cases (frequent splitting or merging of large lists, lock-free concurrency, pure functional programming...), you should _prefer `Vec` or `VecDeque`_ over `LinkedList` - due to less frequent allocation, lower memory overhead, efficient random access, and CPU cache locality.

This example demonstrates common operations with `LinkedList`:

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/linkedlist/linkedlist.rs:example}}
```

## References {#references .skip}

- [Learn Rust by Writing Entirely Too Many Linked Lists][book~rust-unofficial-too-many-lists]↗.

## Related Topics {#related-topics .skip}

- [[vectors | Vectors]].
- [[stack_and_queue | Stacks and Queues]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
