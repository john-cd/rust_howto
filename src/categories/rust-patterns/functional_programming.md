# Functional programming

{{#include functional_programming.incl.md}}

## Compose iterators {#compose-iterators-with-itertools}

[![itertools][c-itertools-badge]][c-itertools]{{hi:itertools}}{{hi:Iterators}}
[![itertools-crates.io][c-itertools-crates.io-badge]][c-itertools-crates.io]
[![itertools-github][c-itertools-github-badge]][c-itertools-github]
[![itertools-lib.rs][c-itertools-lib.rs-badge]][c-itertools-lib.rs]
[![cat-algorithms][cat-algorithms-badge]][cat-algorithms]{{hi:Algorithms}}
[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}
[![cat-no-std::no-alloc][cat-no-std::no-alloc-badge]][cat-no-std::no-alloc]{{hi:No dynamic allocation}}

`itertools` includes extra iterator adaptors, functions and macros.

```rust,editable
{{#include ../../../crates/ex/categories/opqr/tests/rust_patterns/itertools.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[functional_programming: cover the most useful functions (P1)](https://github.com/john-cd/rust_howto/issues/467)

[functional_programming: review - lens is not used that often (P1)](https://github.com/john-cd/rust_howto/issues/468)

## Use `lens` {#lens}

[![lens_rs][c-lens_rs-badge]][c-lens_rs]{{hi:lens_rs}}
[![lens_rs-crates.io][c-lens_rs-crates.io-badge]][c-lens_rs-crates.io]
[![lens_rs-github][c-lens_rs-github-badge]][c-lens_rs-github]
[![lens_rs-lib.rs][c-lens_rs-lib.rs-badge]][c-lens_rs-lib.rs]

This Rust library provides support for lenses, which are a mechanism in functional programming for focusing on a part of a complex data structure.

```rust,editable
{{#include ../../../crates/ex/categories/opqr/tests/rust_patterns/lens.rs:example}}
```

</div>
