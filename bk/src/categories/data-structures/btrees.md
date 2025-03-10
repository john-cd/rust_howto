# BTreeMap and BTreeSet

{{#include btrees.incl.md}}

## `BTreeMap<K, V>` {#btreemap}

[![std][c-std-badge]][c-std] [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]

`BTreeMap<K, V>` is a sorted map data structure, similar to `HashMap`, but its keys are always kept in sorted order. This allows for efficient range queries (e.g., retrieving all values within a specific key range) and ordered iteration. Iterating over a BTreeMap will always yield the key-value pairs in ascending order of the keys.

This ordering is the main difference between `BTreeMap` and the more common `HashMap`. `BTreeMap` is implemented as a B-tree, a self-balancing tree structure that guarantees logarithmic time complexity for most operations.

- `insert(key, value)` inserts a new key-value pair. If the key already exists, the old value is replaced and returned.
- `get(key)` returns a reference to the value associated with the given key, or `None` if the key is not present.
- `remove(key)` removes the key-value pair associated with the given key. Returns the removed value, or None if the key was not present.
- `contains_key(key)` returns true if the map contains the given key.
- `iter()` returns an iterator over the key-value pairs in sorted order.
- `range(range)` returns an iterator over a specified range of keys.
- `first_key_value()` returns the smallest (first) key-value pair.
- `last_key_value()` returns the largest (last) key-value pair.

```rust,editable
{{#include ../../../crates/standard_library/tests/data_structures/btreemap.rs:example}}
```

## `BTreeSet<T>` {#btreeset}

[![std][c-std-badge]][c-std] [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]

`BTreeSet` is a sorted set based on a self-balancing tree, specifically a B-Tree. BTreeSet allows you to store unique elements in a sorted order and provides efficient operations for insertion, deletion, and lookup, with average time complexity of O(log n).

B-Tree Set is similar to `HashSet`, but it keeps the elements sorted.

```rust,editable
{{#include ../../../crates/standard_library/tests/data_structures/btreeset.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write
Add links to BTreeMap / BTreeSet examples in [`std`][c-std]⮳{{hi:std}} lib section
</div>
