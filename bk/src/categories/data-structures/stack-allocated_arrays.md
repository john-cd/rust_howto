# Stack-allocated Arrays

{{#include stack-allocated_arrays.incl.md}}

Stack-allocated arrays are arrays that are stored on the stack, as opposed to the heap. The stack is a region of memory that is used for storing local variables and function call information. Stack allocation has several important characteristics:

- Allocating and deallocating memory on the stack is very fast, because it simply involves adjusting the stack pointer. There is no need for complex memory management as with heap allocation.
- The size of stack-allocated arrays must be known at compile time. This means one cannot resize these arrays dynamically as one can with heap-allocated arrays.
- The stack is typically much smaller than the heap, so stack-allocated arrays are only suitable for small to moderately sized arrays.
- Stack-allocated arrays are automatically deallocated when they go out of scope.

## Store Fixed-size Vectors and Strings on the Stack with `arrayvec` {#arrayvec}

[![arrayvec][c~arrayvec~docs~badge]][c~arrayvec~docs] [![arrayvec~crates.io][c~arrayvec~crates.io~badge]][c~arrayvec~crates.io] [![arrayvec~repo][c~arrayvec~repo~badge]][c~arrayvec~repo] [![arrayvec~lib.rs][c~arrayvec~lib.rs~badge]][c~arrayvec~lib.rs]{{hi:arrayvec}}{{hi:Array}}{{hi:Data-structure}}{{hi:No_std}}{{hi:Stack}}{{hi:Vector}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`arrayvec`][c~arrayvec~docs]↗{{hi:arrayvec}} provides the types `ArrayVec` and `ArrayString`, which are stack-allocated, fixed-capacity array-backed vector and string types.

`ArrayVec` is a vector-like collection with a fixed capacity that is determined at compile time. `ArrayVec` allocates its storage on the stack rather than on the heap, which can lead to better performance. It offers a simple API but also dereferences to a `slice`, so that the full `slice` API is available:

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/stack_allocated/arrayvec.rs:example}}
```

## Store Small Vectors on the Stack with Fallback to the Heap, with `smallvec` {#smallvec}

[![smallvec][c~smallvec~docs~badge]][c~smallvec~docs] [![smallvec~crates.io][c~smallvec~crates.io~badge]][c~smallvec~crates.io] [![smallvec~repo][c~smallvec~repo~badge]][c~smallvec~repo] [![smallvec~lib.rs][c~smallvec~lib.rs~badge]][c~smallvec~lib.rs]{{hi:smallvec}}{{hi:No_std}}{{hi:Small}}{{hi:Stack}}{{hi:Vec}}{{hi:Vector}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

[`smallvec`][c~smallvec~docs]↗{{hi:smallvec}} provides a vector that can store a small number of elements on the stack.
Arrays that are stack-allocated will fallback to the heap if the fixed stack capacity is exceeded.

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/stack_allocated/smallvec.rs:example}}
```

## Store Small Vectors on the Stack with Fallback to the Heap, with `tinyvec` {#tinyvec}

[![tinyvec][c~tinyvec~docs~badge]][c~tinyvec~docs] [![tinyvec~crates.io][c~tinyvec~crates.io~badge]][c~tinyvec~crates.io] [![tinyvec~repo][c~tinyvec~repo~badge]][c~tinyvec~repo] [![tinyvec~lib.rs][c~tinyvec~lib.rs~badge]][c~tinyvec~lib.rs]{{hi:tinyvec}}{{hi:No-std}}{{hi:No_std}}{{hi:Vec}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

The [`tinyvec`][c~tinyvec~docs]↗{{hi:tinyvec}} crate provides a way to work with vectors that can store a small number of elements inline, without heap allocation, and dynamically grow to the heap if necessary. It is in 100% safe Rust code.

`tinyvec` is similar to [`smallvec`][c~smallvec~docs]↗{{hi:smallvec}} but with a smaller feature set and no dependencies.

Note that [`tinyvec`][c~tinyvec~docs]↗{{hi:tinyvec}} requires items to implement the [`Default`][c~std::default::Default~docs]↗{{hi:std::default::Default}} trait.

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/stack_allocated/tinyvec.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[heapless | Heapless data structures]].
- [[slices | Slices]].
- [[vectors | Vectors]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
