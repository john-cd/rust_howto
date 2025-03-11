# Functional programming

{{#include functional_programming.incl.md}}

Functional programming is a programming paradigm that treats computation as the evaluation of mathematical functions and avoids changing-state and mutable data.

Rust is not a purely functional language but incorporates many features from functional programming. By default, variables in Rust are immutable. Rust treats functions as first-class citizens, meaning they can be passed as arguments to other functions, returned from functions, and assigned to variables. Closures, which are anonymous functions that can capture their environment, are also a powerful tool for functional programming in Rust. Other functional features include pattern matching, algebraic data types, and higher-order functions, such as `map`, `filter`, and `fold`.

The following explores some notable Rust crates that aid in functional programming.

## Compose iterators {#compose-iterators-with-itertools}

[![itertools][c-itertools-badge]][c-itertools]{{hi:itertools}}{{hi:Iterators}}
[![itertools-crates.io][c-itertools-crates.io-badge]][c-itertools-crates.io]
[![itertools-github][c-itertools-github-badge]][c-itertools-github]
[![itertools-lib.rs][c-itertools-lib.rs-badge]][c-itertools-lib.rs]
[![cat-algorithms][cat-algorithms-badge]][cat-algorithms]{{hi:Algorithms}}
[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}
[![cat-no-std::no-alloc][cat-no-std::no-alloc-badge]][cat-no-std::no-alloc]{{hi:No dynamic allocation}}

[`itertools`][c-itertools]⮳{{hi:itertools}} includes extra iterator adapters, functions and macros.

It offers a wide range of functions for combining, grouping, and manipulating iterators, for example `itertools::zip_longest`, `itertools::group_by`.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/tests/rust_patterns/itertools.rs:example}}
```

## `either` {#either}

[![either][c-either-badge]][c-either] [![either-crates.io][c-either-crates.io-badge]][c-either-crates.io] [![either-github][c-either-github-badge]][c-either-github] [![either-lib.rs][c-either-lib.rs-badge]][c-either-lib.rs]{{hi:either}}{{hi:Data-structure}}{{hi:No_std}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

The enum [`Either`][c-either]⮳{{hi:Either}} with variants `Left` and `Right` is a general purpose sum type with two cases.

Useful for representing values that can take on one of two different forms, which is a common pattern in functional programming.

`Either` has methods that are similar to `Option` and `Result`, and it also implements traits like `Iterator`.

Includes macros `try_left!()` and `try_right!()` to use for short-circuiting logic, similar to how the `?` operator is used with `Result`. Note that `Either` is general purpose. For describing success or error, use the regular `Result`.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[functional_programming: cover the most useful functions](https://github.com/john-cd/rust_howto/issues/467)
review - lens is not used that often

## Use `lens` {#lens}

[![lens_rs][c-lens_rs-badge]][c-lens_rs]{{hi:lens_rs}}
[![lens_rs-crates.io][c-lens_rs-crates.io-badge]][c-lens_rs-crates.io]
[![lens_rs-github][c-lens_rs-github-badge]][c-lens_rs-github]
[![lens_rs-lib.rs][c-lens_rs-lib.rs-badge]][c-lens_rs-lib.rs]

The `lens` Rust library provides support for lenses, which are a mechanism in functional programming for focusing on a part of a complex data structure.

functional:

Purpose: Provides a set of functional programming abstractions and utilities.
Benefits: Offers tools for composing functions, currying, and other functional patterns.
Note: This crate is less commonly used than itertools, but still offers some useful functional programming tools.

frunk:

Purpose: Provides functional programming data structures and type-level programming tools.
Benefits: Enables advanced functional patterns, such as generic programming with HLists (heterogeneous lists).
Use cases: Useful for complex data transformations and metaprogramming.

im (Immutable Data Structures):

Purpose: Provides immutable data structures, such as lists, maps, and sets.
Benefits: Facilitates functional programming by providing data structures that cannot be modified in place, ensuring immutability.
When to use: Crucial when you need to ensure that data does not change in unexpected ways.

monad:

Purpose: Provides monad implementations and utilities.
Benefits: Enables the use of monads, which are a powerful abstraction for sequencing computations.
Note: Monad concepts are more advanced functional programming concepts.

How to Use Them:

itertools: Often used with iterator chains, enhancing the standard iterator functions.
im: Used to replace standard collections when immutability is required.
frunk: Used for very complex functional programming needs, especially those involving type level programming.
</div>
