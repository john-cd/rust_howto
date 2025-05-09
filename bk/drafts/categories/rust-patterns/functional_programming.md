# Functional Programming

{{#include functional_programming.incl.md}}

Functional programming is a programming paradigm that treats computation as the evaluation of mathematical functions and avoids changing-state and mutable data.

Rust is not a purely functional language, but incorporates many features from functional programming. By default, variables in Rust are immutable. Rust treats functions as first-class citizens, meaning they can be passed as arguments to other functions, returned from functions, and assigned to variables. Closures, which are anonymous functions that can capture their environment, are also a powerful tool for functional programming in Rust. Other functional features include pattern matching, algebraic data types, and higher-order functions, such as `map`, `filter`, and `fold`.

The following explores some notable Rust crates that aid in functional programming:

- [`itertools`][c-itertools]⮳{{hi:itertools}} is often used with iterator chains, enhancing the standard iterator functions.
- [`im`][c-im]⮳{{hi:im}} replaces standard collections when immutability is required.
- [`frunk`][c-frunk]⮳{{hi:frunk}} is used for complex functional programming needs, especially those involving type-level programming.

## Compose Iterators {#compose-iterators-with-itertools}

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
{{#include ../../../crates/cats/rust_patterns/tests/functional_programming/itertools.rs:example}}
```

## Create Immutable Data Structures with `im` {#im}

[![im-website][c-im-website-badge]][c-im-website] [![im][c-im-badge]][c-im] [![im-crates.io][c-im-crates.io-badge]][c-im-crates.io] [![im-github][c-im-github-badge]][c-im-github] [![im-lib.rs][c-im-lib.rs-badge]][c-im-lib.rs]{{hi:im}}{{hi:Persistent}}{{hi:Hamt}}{{hi:Immutable}}{{hi:B-tree}}{{hi:Rrb-tree}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}

[`im`][c-im]⮳{{hi:im}} provides immutable data structures, such as lists, [maps][p-maps], and sets. It facilitates functional programming by providing data structures that cannot be modified in place. Use it when you need to ensure that data does not change in unexpected ways.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/tests/functional_programming/im.rs:example}}
```

## `rpds` {#rpds}

[![rpds][c-rpds-badge]][c-rpds] [![rpds-crates.io][c-rpds-crates.io-badge]][c-rpds-crates.io] [![rpds-github][c-rpds-github-badge]][c-rpds-github] [![rpds-lib.rs][c-rpds-lib.rs-badge]][c-rpds-lib.rs]{{hi:rpds}}{{hi:Data-structure}}{{hi:No_std}}{{hi:Persistent}}{{hi:Immutable}}{{hi:Data-structures}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}

`rpds` provides persistent data structures with structural sharing.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/tests/functional_programming/rpds.rs:example}}
```

## Use a general purpose sum type with `either` {#either}

[![either][c-either-badge]][c-either] [![either-crates.io][c-either-crates.io-badge]][c-either-crates.io] [![either-github][c-either-github-badge]][c-either-github] [![either-lib.rs][c-either-lib.rs-badge]][c-either-lib.rs]{{hi:either}}{{hi:Data-structure}}{{hi:No_std}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

The enum [`Either`][c-either]⮳{{hi:Either}} with variants `Left` and `Right` is a general purpose sum type with two cases. This is useful for representing values that can take on one of two different forms, which is a common pattern in functional programming. [`Either`][c-either]⮳{{hi:Either}} has methods that are similar to `Option` and `Result`, and it also implements traits like `Iterator`. This crate also includes [macros][p-macros] `try_left!()` and `try_right!()` to use for short-circuiting logic, similar to how the `?` operator is used with `Result`.

Note that [`Either`][c-either]⮳{{hi:Either}} is general purpose. For describing success or error, use the regular `Result` enum.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/tests/functional_programming/either.rs:example}}
```

## Use Functional Programming Data Structures and Type-level Programming Tools with `frunk` {#frunk}

[![frunk][c-frunk-badge]][c-frunk] [![frunk-crates.io][c-frunk-crates.io-badge]][c-frunk-crates.io] [![frunk-github][c-frunk-github-badge]][c-frunk-github] [![frunk-lib.rs][c-frunk-lib.rs-badge]][c-frunk-lib.rs]{{hi:frunk}}{{hi:Monoid}}{{hi:Generic}}{{hi:HList}}{{hi:Validated}}{{hi:frunk}}

Frunk is a functional programming toolbelt for Rust. It provides developers with a number of functional programming data structures and type-level programming tools like `HList` (heterogeneous lists), `Coproduct`, `Generic`, `LabelledGeneric`, `Validated`, `Monoid`, `Semigroup` and friends. It is useful for complex data transformations and metaprogramming.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/tests/functional_programming/frunk.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[functional_programming: organize / align intro and sections / add examples NOW](https://github.com/john-cd/rust_howto/issues/467)
review https://geo-ant.github.io/blog/2023/rust-type-level-programming/
https://nota-lang.org/examples/blog-post/standalone/
review https://crates.io/crates/tap
ADD Working with Iterators: Creating custom iterators, using iterator adapters, and understanding iterator traits. Here or in std lib?

- [Type-level Programming in Rust](https://willcrichton.net/notes/type-level-programming/)

</div>
