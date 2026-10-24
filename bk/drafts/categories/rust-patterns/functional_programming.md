<!-- TODO Reviewed ??-??-2026 -->

# Functional Programming

{{#include functional_programming.incl.md}}

Functional style builds results by composing small functions over values. Rust supports this style with functions, closures, pattern matching, and lazy iterator adapters such as [`map`][c~std::iter::Iterator::map~docs]↗{{hi:map}}, [`filter`][c~std::iter::Iterator::filter~docs]↗{{hi:filter}}, and [`fold`][c~std::iter::Iterator::fold~docs]↗{{hi:fold}}. Rust is multi-paradigm: mutation is available when it makes the code clearer, while ownership and borrowing still govern how values are shared.

Start with the standard iterator adapters, then use [`itertools`][c~itertools~docs]↗{{hi:itertools}} when its additional adapters make a transformation clearer. Persistent collections such as [`imbl`][c~imbl~docs]↗{{hi:imbl}} and [`rpds`][c~rpds~docs]↗{{hi:rpds}} can share structure between versions. [`either`][c~either~docs]↗{{hi:either}} represents one of two alternatives, and [`frunk`][c~frunk~docs]↗{{hi:frunk}} provides heterogeneous lists and type-level tools.

## Transform Collections with Iterator Adapters {#iterator-adapters}

Iterator adapters are lazy: `filter` and `map` describe a transformation, and a consumer such as `fold` runs it. Closures let each step capture values such as a threshold without introducing mutable intermediate collections.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/functional_programming/iterators.rs:example}}
```

## Compose Iterators {#compose-iterators-with-itertools}

[![itertools][c~itertools~docs~badge]][c~itertools~docs]{{hi:itertools}}{{hi:Iterators}}
[![itertools~crates.io][c~itertools~crates.io~badge]][c~itertools~crates.io]
[![itertools~repo][c~itertools~repo~badge]][c~itertools~repo]
[![itertools~lib.rs][c~itertools~lib.rs~badge]][c~itertools~lib.rs]
[![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}
[![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}
[![cat~no-std::no-alloc][cat~no-std::no-alloc~badge]][cat~no-std::no-alloc]{{hi:No dynamic allocation}}

[`itertools`][c~itertools~docs]↗{{hi:itertools}} includes extra iterator adapters, functions and macros.

It offers a wide range of functions for combining, grouping, and manipulating iterators, for example `itertools::zip_longest`, `itertools::group_by`.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/functional_programming/itertools.rs:example}}
```

## Create Persistent Collections with `imbl` {#im}

[![imbl][c~imbl~docs~badge]][c~imbl~docs] [![imbl][c~imbl~crates.io~badge]][c~imbl~crates.io]{{hi:imbl}}{{hi:Persistent}}{{hi:Hamt}}{{hi:Immutable}}{{hi:B-tree}}{{hi:Rrb-tree}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

[`imbl`][c~imbl~docs]↗{{hi:imbl}} provides persistent collections such as vectors, maps, and sets. Updates produce a new version that shares unchanged structure with the old one. This is useful when keeping multiple related versions is more important than in-place mutation. The example imports the crate by its package name, `imbl`.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/functional_programming/imbl.rs:example}}
```

## Use Persistent Collections with `rpds` {#rpds}

[![rpds][c~rpds~docs~badge]][c~rpds~docs] [![rpds~crates.io][c~rpds~crates.io~badge]][c~rpds~crates.io] [![rpds~repo][c~rpds~repo~badge]][c~rpds~repo] [![rpds~lib.rs][c~rpds~lib.rs~badge]][c~rpds~lib.rs]{{hi:rpds}}{{hi:Data-structure}}{{hi:No_std}}{{hi:Persistent}}{{hi:Immutable}}{{hi:Data-structures}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

[`rpds`][c~rpds~docs]↗{{hi:rpds}} provides persistent data structures with structural sharing. Like `imbl`, it can make it inexpensive to retain related versions of a collection; choose a collection based on its API and workload.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/functional_programming/rpds.rs:example}}
```

## Represent Two Alternatives with `either` {#either}

[![either][c~either~docs~badge]][c~either~docs] [![either~crates.io][c~either~crates.io~badge]][c~either~crates.io] [![either~repo][c~either~repo~badge]][c~either~repo] [![either~lib.rs][c~either~lib.rs~badge]][c~either~lib.rs]{{hi:either}}{{hi:Data-structure}}{{hi:No_std}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

The enum [`Either`][c~either~docs]↗{{hi:Either}} with variants `Left` and `Right` is a general purpose sum type with two cases. This is useful for representing values that can take on one of two different forms, which is a common pattern in functional programming. [`Either`][c~either~docs]↗{{hi:Either}} has methods that are similar to `Option` and `Result`, and it also implements traits like `Iterator`. This crate also includes [macros][p~macros] `try_left!()` and `try_right!()` to use for short-circuiting logic, similar to how the `?` operator is used with `Result`.

Note that [`Either`][c~either~docs]↗{{hi:Either}} is general purpose. For describing success or error, use the regular [`Result`][c~std::result::Result~docs]↗{{hi:std::result::Result}} enum.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/functional_programming/either.rs:example}}
```

## Work with Heterogeneous Lists and Type-level Tools with `frunk` {#frunk}

[![frunk][c~frunk~docs~badge]][c~frunk~docs] [![frunk~crates.io][c~frunk~crates.io~badge]][c~frunk~crates.io] [![frunk~repo][c~frunk~repo~badge]][c~frunk~repo] [![frunk~lib.rs][c~frunk~lib.rs~badge]][c~frunk~lib.rs]{{hi:frunk}}{{hi:Monoid}}{{hi:Generic}}{{hi:HList}}{{hi:Validated}}{{hi:frunk}}

Frunk is a functional programming toolbelt for Rust. It provides developers with a number of functional programming data structures and type-level programming tools like [`HList`][c~frunk::hlist::HList~docs]↗{{hi:HList}} (heterogeneous lists)↗, `Coproduct`, `Generic`, `LabelledGeneric`, `Validated`, `Monoid`, `Semigroup` and friends. It is useful for complex data transformations and metaprogramming.

```rust,editable,noplayground
{{#include ../../../crates/cats/rust_patterns/examples/functional_programming/frunk.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Iterators][book~rust~iterators] and [closures][book~rust~closures] in the Rust Book.
- [[structural_patterns | Structural Patterns]] for pattern matching and algebraic data types.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[functional_programming: organize / align intro and sections / add examples](https://github.com/john-cd/rust_howto/issues/467)
review
- [rust-type-level-programming][blog~rust-type-level-programming]↗.
- [Type-Level Programming in Rust][blog~type-level-programming-in-rust]↗.

review [tap][c~tap~crates.io]↗.

- [Type-level Programming in Rust][blog~willcrichton-type-level-programming]↗.

</div>
