# Binary Heaps

{{#include binaryheap.incl.md}}

## Implement a Priority Queue {#skip}

[![std][c~std~docs~badge]][c~std~docs] [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/binaryheap/binaryheap.rs:example}}
```

## `priority-queue` {#priority-queue}

[![priority-queue][c~priority_queue~docs~badge]][c~priority_queue~docs] [![priority-queue~crates.io][c~priority_queue~crates.io~badge]][c~priority_queue~crates.io] [![priority-queue~github][c~priority_queue~github~badge]][c~priority_queue~github] [![priority-queue~lib.rs][c~priority_queue~lib.rs~badge]][c~priority_queue~lib.rs]{{hi:priority-queue}}{{hi:Heap}}{{hi:Priority}}{{hi:Queue}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

[`priority-queue`][c~priority_queue~docs]⮳{{hi:priority-queue}} implements a Priority Queue implemented as a heap with a function to efficiently change the priority of an item. Priority and items are stored in an IndexMap and the queue is implemented as a Heap of indexes.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/binaryheap/priority_queue.rs:example}}
```

## Related Topics

- [[btrees | B-trees]].
- [[maps | Maps]].
- [[stack_and_queue | Stack and Queue]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/1172)
</div>
