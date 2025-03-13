# Stack-allocated arrays

{{#include stack_allocated_arrays.incl.md}}

Stack-allocated arrays are arrays that are stored on the stack, as opposed to the heap.
The stack is a region of memory that is used for storing local variables and function call information.
Stack allocation has several important characteristics:

- The size of stack-allocated arrays must be known at compile time. This means you cannot resize these arrays dynamically as you can with heap-allocated arrays.
- Allocating and deallocating memory on the stack is very fast, because it simply involves adjusting the stack pointer. There is no need for complex memory management as with heap allocation.
- The stack is typically much smaller than the heap, so stack-allocated arrays are suitable for small to moderately sized arrays.
- Stack-allocated arrays are automatically deallocated when they go out of scope.

## `arrayvec` {#arrayvec}

[![arrayvec][c-arrayvec-badge]][c-arrayvec]{{hi:arrayvec}}
[![arrayvec-crates.io][c-arrayvec-crates.io-badge]][c-arrayvec-crates.io]
[![arrayvec-github][c-arrayvec-github-badge]][c-arrayvec-github]
[![arrayvec-lib.rs][c-arrayvec-lib.rs-badge]][c-arrayvec-lib.rs]
[![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

[`arrayvec`][c-arrayvec]⮳{{hi:arrayvec}} provides the types `ArrayVec` and `ArrayString`, which are stack-allocated, fixed size array-backed vector and string types.

`ArrayVec` is a vector-like collection with a fixed capacity that is determined at compile time.
`ArrayVec` allocates its storage on the stack rather than on the heap, which can lead to better performance.
It offers a simple API but also dereferences to a `slice`, so that the full `slice` API is available.

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/tests/vec/arrayvec.rs:example}}
```

## `smallvec` {#smallvec}

[![smallvec][c-smallvec-badge]][c-smallvec]{{hi:smallvec}}
[![smallvec-crates.io][c-smallvec-crates.io-badge]][c-smallvec-crates.io]
[![smallvec-github][c-smallvec-github-badge]][c-smallvec-github]
[![smallvec-lib.rs][c-smallvec-lib.rs-badge]][c-smallvec-lib.rs]
[![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}

[`smallvec`][c-smallvec]⮳{{hi:smallvec}} provides a vector that can store a small number of elements on the stack.
Arrays that are stack-allocated will fallback to the heap if the fixed stack capacity is exceeded.

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/tests/vec/smallvec.rs:example}}
```

## `tinyvec` {#tinyvec}

[![tinyvec][c-tinyvec-badge]][c-tinyvec]{{hi:tinyvec}}
[![tinyvec-crates.io][c-tinyvec-crates.io-badge]][c-tinyvec-crates.io]
[![tinyvec-github][c-tinyvec-github-badge]][c-tinyvec-github]
[![tinyvec-lib.rs][c-tinyvec-lib.rs-badge]][c-tinyvec-lib.rs]
[![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

The [`tinyvec`][c-tinyvec]⮳{{hi:tinyvec}} crate provides a way to work with vectors that can store a small number of elements inline, without heap allocation, and dynamically grow to the heap if necessary.
It is in 100% safe Rust code. It is similar to [`smallvec`][c-smallvec]⮳{{hi:smallvec}} but with a smaller feature set and no dependencies.
[`tinyvec`][c-tinyvec]⮳{{hi:tinyvec}} requires items to implement the [`Default`][c-std::default::Default]⮳{{hi:Default}} trait.

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/tests/vec/tinyvec.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[stack_allocated_arrays: write](https://github.com/john-cd/rust_howto/issues/282)
</div>
