# B-Trees: Sorted Maps and Sets

{{#include btrees.incl.md}}

## Create a Map Sorted by Key with `BTreeMap` {#btreemap}

[![std][c~std~docs~badge]][c~std~docs] [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}{{hi:B-trees}}

[`BTreeMap<K, V>`][c~std::collections::BTreeMap~docs]↗{{hi:std::collections::BTreeMap}} is a sorted map data structure, similar to `HashMap`, but its keys are always kept in sorted order. This allows for efficient range queries (e.g., retrieving all values within a specific key range) and ordered iteration. Iterating over a `BTreeMap` will always yield the key-value pairs in ascending order of the keys.

`BTreeMap` is implemented as a B-tree, a self-balancing tree structure that guarantees logarithmic time complexity for most operations. The following are common operations:

- `insert(key, value)` inserts a new key-value pair. If the key already exists, the old value is replaced and returned.
- `get(key)` returns a reference to the value associated with the given key, or `None` if the key is not present.
- `remove(key)` removes the key-value pair associated with the given key. It returns the removed value, or `None` if the key was not present.
- `contains_key(key)` returns `true` if the map contains the given key.
- `iter()` returns an iterator over the key-value pairs, in sorted order.
- `range(range)` returns an iterator over a specified range of keys.
- `first_key_value()` returns the smallest (first) key-value pair.
- `last_key_value()` returns the largest (last) key-value pair.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/btrees/btreemap.rs:example}}
```

## Create a Sorted Set with `BTreeSet<T>` {#btreeset}

[![std][c~std~docs~badge]][c~std~docs] [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}{{hi:B-trees}}

`BTreeSet` is similar to `HashSet`, but it keeps the elements sorted.

It is based on a self-balancing tree, specifically a B-Tree. `BTreeSet` allows storing unique elements in a sorted order and provides efficient operations for insertion, deletion, and lookup, with average time complexity of O(log n):

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/btrees/btreeset.rs:example}}
```

## Related Topics

- [[hashmaps | Hashmaps]].
- Other [[maps | Maps]].
- [[sorting | Sorting]].
- [[vectors | Vectors]].

Refer as well to the [`ordered-float`][c~ordered-float~docs]↗{{hi:ordered-float}} example in the [[additional_numeric_types | Additional Numeric Types]] chapter.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
