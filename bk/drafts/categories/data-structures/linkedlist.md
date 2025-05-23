# Linked Lists

{{#include linkedlist.incl.md}}

## Stored Data in a Linked List {#linkedlist}

[![std][c-std-badge]][c-std] [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]

`LinkedList` represents a doubly linked list, where each element contains a reference to both the next and the previous elements. This allows for efficient insertions and deletions from both ends of the list.

- You can iterate over the elements of the list in both forward and backward directions.
- Adding or removing elements from the front or back of the list is very efficient, with an average time complexity of O(1).
- The list can grow or shrink as needed.

```rust,editable
{{#include ../../../crates/cats/data_structures/tests/linkedlist/linkedlist.rs:example}}
```

## Related Topics {#skip}

- [[vectors | Vectors]].
- [[stack_and_queue | Stacks and Queues]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[expand](https://github.com/john-cd/rust_howto/issues/1170)
</div>
