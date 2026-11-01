<!-- TODO Reviewed ??-??-2026 -->

# HashMap's Friends: `IndexMap`, `MultiMap`, and `SlotMap`

{{#include other_maps.incl.md}}

## Store Data in an Insertion-ordered Map {#indexmap}

{{#badges indexmap data-structures="Data structures" no-std="No standard library"}}{{hi:Hashmap}}{{hi:No_std}}

[`indexmap`][c~indexmap~docs]↗{{hi:indexmap}} offers the `IndexMap` data structure that combines the features of a hashmap and a vector.
It is a hash table that keeps track of insertion order and allows efficient iteration over its elements in that order.

This example demonstrates the usage of `IndexMap`, including methods for accessing elements by index and using the `entry` API:

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/maps/indexmap.rs:example}}
```

## Store Multiple Values for a Single Key with `multimap` {#multimap}

{{#badges multimap}}

A `MultiMap` allows storing multiple values for a single key, which can be useful to associate several items with the same identifier. [`multimap`][c~multimap~docs]↗ is implemented as a thin wrapper around [`std::collections::HashMap`][c~std::collections::HashMap~docs]↗{{hi:std::collections::HashMap}}.

This example queries the crates.io API for crate categories at run time, so it requires network access.

The following example demonstrates how to use a `MultiMap`:

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/maps/multimap.rs:example}}
```

## Store Collections of Objects that Need Unique, Stable, Safe Identifiers {#slotmap}

{{#badges slotmap caching=Caching data-structures="Data structures" memory-management="Memory management"}}{{hi:Arena}}{{hi:Allocator}}{{hi:Reference}}{{hi:slotmap}}{{hi:Storage}}

Use [`slotmap`][c~slotmap~docs]↗ to store collections of objects that need generated, unique, persistent identifiers ("keys"), but have no clear ownership otherwise, such as game entities or graph nodes.

`slotmap` items can be added or removed dynamically. Slotmap ensures stable indices, meaning once an item is inserted, its key remains valid until the item is explicitly removed.

[`slotmap`][c~slotmap~docs]↗ provides three containers: [`SlotMap`][c~slotmap~docs]↗, `HopSlotMap` and `DenseSlotMap`. Two secondary maps, `SecondaryMap` and [`SparseSecondaryMap`][c~slotmap::SparseSecondaryMap~docs]↗{{hi:SparseSecondaryMap}} are also provided to map additional objects to the keys created by one of the slot maps.

This example showcases how to create, insert, access, and remove elements from a `SlotMap`, and how to use a `SecondaryMap` to associate additional data with the elements in the `SlotMap`:

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/maps/slotmap.rs:example}}
```

## Remember Insertion Order with `linked-hash-map` {#linked-hash-map}

{{#badges linked-hash-map data-structures=}}

[`linked-hash-map`][c~linked-hash-map~docs]↗{{hi:linked-hash-map}} is a hash map that iterates in insertion order. It is older than [`indexmap`](#indexmap) and has not changed much recently: prefer `indexmap` for new code, and use `linked-hash-map` when you need its `LinkedHashMap` API, for example to move an entry to the end when it is used again.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/maps/linked_hash_map.rs:example}}
```

## Map Ranges of Keys to Values with `rangemap` {#rangemap}

{{#badges rangemap data-structures=}}

[`rangemap`][c~rangemap~docs]↗{{hi:rangemap}} stores values against ranges of keys instead of single keys, and merges or splits ranges as you insert. Range ends are exclusive, so `0..50` and `50..100` are adjacent and `100` is not covered. It suits address spaces, schedules and version ranges.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/rangemap/rangemap.rs:example}}
```

## Keep a Sorted Map in a Skip List with `skiplist` {#skiplist}

{{#badges skiplist data-structures=}}

[`skiplist`][c~skiplist~docs]↗{{hi:skiplist}} provides `SkipMap`, an ordered map built on a skip list. Iteration visits the keys in sorted order regardless of insertion order. For most programs, [`BTreeMap`][c~std::collections::BTreeMap~docs]↗ from the standard library is the simpler choice; a skip list is mainly of interest for its structure and for `skiplist`'s ordered-collection API.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/skiplist/skiplist.rs:example}}
```

## Test Membership Probabilistically with `bloomfilter` {#bloomfilter}

{{#badges bloomfilter data-structures=}}

A Bloom filter is a compact set that can say "possibly present" or "definitely absent": it may report false positives but never false negatives. [`bloomfilter`][c~bloomfilter~docs]↗{{hi:bloomfilter}} sizes the filter from the number of items you expect and the false-positive rate you accept. Use it to skip expensive lookups for keys that are certainly not in a store.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/bloomfilter/bloomfilter.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[graphs | Graphs]].
- [[hashmaps | Hashmaps]].
- [[vectors | Vectors]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  accuracy review: data structures
</div>
