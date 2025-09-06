# Binary Heaps and Priority Queues

{{#include binary_heaps.incl.md}}

## Work with a Binary Heap {#binary-heap}

[![std][c~std~docs~badge]][c~std~docs] [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]

`std::collections::BinaryHeap` is a binary heap, where elements can be efficiently inserted and the maximum or minimum element can be quickly accessed. By default, `BinaryHeap` is a "max-heap", meaning the largest element is always at the top. The following demonstrates its basic operations:

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/binaryheap/binaryheap.rs:example}}
```

## Create a Priority Queue {#priority-queue}

[![priority-queue][c~priority-queue~docs~badge]][c~priority-queue~docs] [![priority-queue~crates.io][c~priority-queue~crates.io~badge]][c~priority-queue~crates.io] [![priority-queue~repo][c~priority-queue~repo~badge]][c~priority-queue~repo] [![priority-queue~lib.rs][c~priority-queue~lib.rs~badge]][c~priority-queue~lib.rs]{{hi:priority-queue}}{{hi:Heap}}{{hi:Priority}}{{hi:Queue}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

A priority queue is a type of data structure where each element is assigned a priority, and elements with higher priority are processed before those with lower priority. Common use cases include:

- Managing tasks where the highest-priority task must always be processed next.
- Graph algorithms like Dijkstra's or A*.
- Handling events in chronological order.
- Efficiently finding the K largest items in a stream of data without sorting the entire set.

The [`priority-queue`][c~priority-queue~docs]↗{{hi:priority-queue}} crate provides a more versatile priority queue than `std::collections::BinaryHeap`, most notably allowing you to assign separate priorities to items.

This example demonstrates the usage of the `priority-queue` crate in Rust. It showcases various functionalities including creating a priority queue,
inserting elements, popping elements, using custom types, and more:

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/binaryheap/priority_queue.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[b-trees | B-trees]].
- [[other_maps | Maps]].
- [[stacks_and_queues | Stacks and Queues]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
