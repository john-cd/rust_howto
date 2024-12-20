# Stack-allocated arrays

{{#include stack_allocated_arrays.incl.md}}

## `arrayvec` {#arrayvec}

[![arrayvec][c-arrayvec-badge]][c-arrayvec]{{hi:arrayvec}}
[![arrayvec-crates.io][c-arrayvec-crates.io-badge]][c-arrayvec-crates.io]
[![arrayvec-github][c-arrayvec-github-badge]][c-arrayvec-github]
[![arrayvec-lib.rs][c-arrayvec-lib.rs-badge]][c-arrayvec-lib.rs]
[![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Arrays that are ONLY stack-allocated with fixed capacity.

```rust,editable,noplayground
{{#include ../../../crates/ex/categories/d/tests/data_structures/arrayvec.rs:example}}
```

## `smallvec` {#smallvec}

[![smallvec][c-smallvec-badge]][c-smallvec]{{hi:smallvec}}
[![smallvec-crates.io][c-smallvec-crates.io-badge]][c-smallvec-crates.io]
[![smallvec-github][c-smallvec-github-badge]][c-smallvec-github]
[![smallvec-lib.rs][c-smallvec-lib.rs-badge]][c-smallvec-lib.rs]
[![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}

Arrays that are stack-allocated with fallback to the heap if the fixed stack capacity is exceeded.

```rust,editable,noplayground
{{#include ../../../crates/ex/categories/d/tests/data_structures/smallvec.rs:example}}
```

## `tinyvec` {#tinyvec}

[![tinyvec][c-tinyvec-badge]][c-tinyvec]{{hi:tinyvec}}
[![tinyvec-crates.io][c-tinyvec-crates.io-badge]][c-tinyvec-crates.io]
[![tinyvec-github][c-tinyvec-github-badge]][c-tinyvec-github]
[![tinyvec-lib.rs][c-tinyvec-lib.rs-badge]][c-tinyvec-lib.rs]
[![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

The `tinyvec` crate provides a way to work with vectors that can store a small number of elements inline, without heap allocation, and dynamically grow to the heap if necessary. It is in 100% safe Rust code. It's similar to `smallvec` but with a smaller feature set and no dependencies. `tinyvec` requires items to implement the `Default` trait.

```rust,editable,noplayground
{{#include ../../../crates/ex/categories/d/tests/data_structures/tinyvec.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[stack_allocated_arrays: write (P1)](https://github.com/john-cd/rust_howto/issues/282)

- Review <https://docs.rs/tinyvec/latest/tinyvec/>

</div>
