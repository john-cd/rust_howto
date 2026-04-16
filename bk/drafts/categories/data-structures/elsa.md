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
// Add to Cargo.toml:
// [dependencies]
// elsa = "1.11"

use elsa::FrozenVec;

fn main() {
    let frozen: FrozenVec<Box<String>> = FrozenVec::new();

    // `push_get` inserts an element and returns a reference to it.
    // This reference remains valid even after further insertions.
    let first: &String = frozen.push_get(Box::new("first".to_string()));
    let second: &String = frozen.push_get(Box::new("second".to_string()));
    let third: &String = frozen.push_get(Box::new("third".to_string()));

    // Importantly, `first` and `second` are still valid even though we added `third`:
    println!("first:  {first}");
    println!("second: {second}");
    println!("third:  {third}");
    println!("total:  {}", frozen.len());

    // `FrozenVec` also supports indexing:
    println!("index 0: {}", frozen[0]);
}
```

A `FrozenMap<K, Box<V>>` works similarly for key-value pairs:

```rust,noplayground
// Add to Cargo.toml:
// [dependencies]
// elsa = "1.11"

use elsa::FrozenMap;

fn main() {
    let map: FrozenMap<&str, Box<u32>> = FrozenMap::new();

    map.insert("one", Box::new(1));
    map.insert("two", Box::new(2));

    // References remain stable after further insertions:
    let one: &u32 = map.get("one").unwrap();
    map.insert("three", Box::new(3));
    println!("one is still: {one}");
}
```

## Related Topics {#related-topics .skip}

- [[vectors | Vectors]].
- [[hashmaps | HashMaps]].
- [[rust-patterns | Rust Patterns]].
- [[concurrent_data_structures | Concurrent Data Structures]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
