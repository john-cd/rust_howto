# Stacks and Queues

{{#include stack_and_queue.incl.md}}

While Rust's standard library doesn't have dedicated `Stack`{{hi:Stack}} and `Queue` types in the same way some other languages do, you can easily implement their functionality using existing data structures, primarily [`Vec`][c~std::vec::Vec~docs] (for stack-like behavior) and [`VecDeque`][c~std::collections::VecDeque~docs] (for queue-like behavior).

## Implement a Stack Using `Vec` {#stack}

[![std][c~std~docs~badge]][c~std~docs] [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]

A stack is a LIFO (Last-In, First-Out) data structure. You can use a [`Vec`][c~std::vec::Vec~docs]↗{{hi:std::vec::Vec}} to mimic a stack, because `Vec` provides efficient `push` (add to the top) and `pop` (remove from the top) operations.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/stack_and_queue/stack.rs:example}}
```

## Implement a Queue Using `VecDeque` {#queue}

[![std][c~std~docs~badge]][c~std~docs] [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]

A queue is a FIFO (First-In, First-Out) data structure. [`VecDeque`][c~std::collections::VecDeque~docs]{{hi:std::collections::VecDeque}} (Vector Deque) is well-suited for implementing queues, because it provides efficient `push_back` (add to the rear) and `pop_front` (remove from the front) operations.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/stack_and_queue/queue.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write NOW](https://github.com/john-cd/rust_howto/issues/1169)
</div>
