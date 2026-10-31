<!-- TODO Reviewed ??-??-2026 -->

# Append-only Collections with `elsa`

{{#include elsa.incl.md}}

[`elsa`][c~elsa~docs]↗{{hi:elsa}} provides "frozen" (append-only) collections: `FrozenVec`, `FrozenMap`, `FrozenBTreeMap`, `FrozenIndexMap`, and related types. These collections provide interior mutability that only allows insertions — you can add elements but never remove, replace, or reorder them.

Key properties of `elsa` collections:

- **Stable references**: Elements are never moved after insertion, so references into the collection remain valid even after new elements are added.
- **Interior mutability**: You can add elements through a shared `&self` reference, without needing `&mut self`.
- **No removals**: Once an element is inserted, it stays for the lifetime of the collection.
- **No reallocations** (for `FrozenVec`): The underlying storage never moves, because `FrozenVec` stores elements in `Box<T>`.

This is particularly useful for:

- Self-referential data structures where elements hold references to each other.
- Caches or memoization tables where you want to retain references to cached values across insertions.
- Interning tables (e.g., interning strings) where stable identity pointers are important.

## Use Append-only Frozen Collections {#elsa-frozen-vec}

[![elsa][c~elsa~docs~badge]][c~elsa~docs] [![elsa~crates.io][c~elsa~crates.io~badge]][c~elsa~crates.io] [![elsa~repo][c~elsa~repo~badge]][c~elsa~repo]{{hi:elsa}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

The following example demonstrates how `FrozenVec` allows inserting elements and holding onto references to them at the same time:

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/elsa/frozen_vec.rs:example}}
```

A `FrozenMap<K, Box<V>>` works similarly for key-value pairs:

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/elsa/frozen_map.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[vectors | Vectors]].
- [[hashmaps | HashMaps]].
- [[rust-patterns | Rust Patterns]].
- [[concurrent_data_structures | Concurrent Data Structures]].

{{#include ../../../src/categories/data-structures/refs.incl.md}}
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
