# Linked Lists

{{#include linkedlist.incl.md}}

## Stored data in a linked list {#linkedlist}

[![std][c-std-badge]][c-std] [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]

`LinkedList` represents a doubly linked list, where each element contains a reference to both the next and the previous elements. This allows for efficient insertions and deletions from both ends of the list.

- You can iterate over the elements of the list in both forward and backward directions.
- Adding or removing elements from the front or back of the list is very efficient, with an average time complexity of O(1).
- The list can grow or shrink as needed.

```rust,editable
{{#include ../../../crates/standard_library/tests/data_structures/linkedlist.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P1 write
Include stack / queue examples

Here or in `std` lib section?
</div>
