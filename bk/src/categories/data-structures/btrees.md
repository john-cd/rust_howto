# BTreeMap and BTreeSet

{{#include btrees.incl.md}}

## `BTreeMap<K, V>` {#btreemap}

[![std][c~std~docs~badge]][c~std~docs] [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]

`BTreeMap<K, V>` is a sorted map data structure, similar to `HashMap`, but its keys are always kept in sorted order. This allows for efficient range queries (e.g., retrieving all values within a specific key range) and ordered iteration. Iterating over a BTreeMap will always yield the key-value pairs in ascending order of the keys.

This ordering is the main difference between [`BTreeMap`][c~std::collections::BTreeMap~docs]{{hi:std::collections::BTreeMap}} and the more common `HashMap`. `BTreeMap` is implemented as a B-tree, a self-balancing tree structure that guarantees logarithmic time complexity for most operations.
[c~std::collections::BTreeMap~docs]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html

- [`insert(key, value)`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.insert)↗{{hi:std::collections::BTreeMap::insert}} inserts a new key-value pair. If the key already exists, the old value is replaced and returned.
- [`get(key)`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.get)↗{{hi:std::collections::BTreeMap::get}} returns a reference to the value associated with the given key, or `None` if the key is not present.
- [`remove(key)`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.remove)↗{{hi:std::collections::BTreeMap::remove}} removes the key-value pair associated with the given key. Returns the removed value, or None if the key was not present.
- [`contains_key(key)`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.contains_key)↗{{hi:std::collections::BTreeMap::contains_key}} returns true if the map contains the given key.
- [`iter()`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.iter)↗{{hi:std::collections::BTreeMap::iter}} returns an iterator over the key-value pairs in sorted order.
- [`range(range)`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.range)↗{{hi:std::collections::BTreeMap::range}} returns an iterator over a specified range of keys.
- [`first_key_value()`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.first_key_value)↗{{hi:std::collections::BTreeMap::first_key_value}} returns the smallest (first)↗ key-value pair.
- [`last_key_value()`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.last_key_value)↗{{hi:std::collections::BTreeMap::last_key_value}} returns the largest (last) key-value pair.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/btrees/btreemap.rs:example}}
```

## `BTreeSet<T>` {#btreeset}

[![std][c~std~docs~badge]][c~std~docs] [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]

`BTreeSet` is a sorted set based on a self-balancing tree, specifically a B-Tree. BTreeSet allows you to store unique elements in a sorted order and provides efficient operations for insertion, deletion, and lookup, with average time complexity of O(log n).

B-Tree Set is similar to `HashSet`, but it keeps the elements sorted.

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
[write](https://github.com/john-cd/rust_howto/issues/1168)
</div>
