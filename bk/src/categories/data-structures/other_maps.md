<!-- TODO Reviewed ??-??-2026 -->

# HashMap's Friends: `IndexMap`, `MultiMap`, and `SlotMap`

{{#include other_maps.incl.md}}

## Store Data in an Insertion-ordered Map {#indexmap}

[![indexmap][c~indexmap~docs~badge]][c~indexmap~docs] [![indexmap~crates.io][c~indexmap~crates.io~badge]][c~indexmap~crates.io] [![indexmap~repo][c~indexmap~repo~badge]][c~indexmap~repo] [![indexmap~lib.rs][c~indexmap~lib.rs~badge]][c~indexmap~lib.rs]{{hi:indexmap}}{{hi:Hashmap}}{{hi:No_std}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`indexmap`][c~indexmap~docs]↗{{hi:indexmap}} offers the `IndexMap` data structure that combines the features of a hashmap and a vector.
It is a hash table that keeps track of insertion order and allows efficient iteration over its elements in that order.

This example demonstrates the usage of `IndexMap`, including methods for accessing elements by index and using the `entry` API:

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/maps/indexmap.rs:example}}
```

## Store Multiple Values for a Single Key with `multimap` {#multimap}

[![multimap][c~multimap~docs~badge]][c~multimap~docs] [![multimap~crates.io][c~multimap~crates.io~badge]][c~multimap~crates.io] [![multimap~repo][c~multimap~repo~badge]][c~multimap~repo] [![multimap~lib.rs][c~multimap~lib.rs~badge]][c~multimap~lib.rs]{{hi:multimap}}

A `MultiMap` allows storing multiple values for a single key, which can be useful to associate several items with the same identifier. [`multimap`][c~multimap~docs]↗ is implemented as a thin wrapper around [`std::collections::HashMap`][c~std::collections::HashMap~docs]↗{{hi:std::collections::HashMap}}.

This example queries the crates.io API for crate categories at run time, so it requires network access.

The following example demonstrates how to use a `MultiMap`:

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/maps/multimap.rs:example}}
```

## Store Collections of Objects that Need Unique, Stable, Safe Identifiers {#slotmap}

[![slotmap][c~slotmap~docs~badge]][c~slotmap~docs] [![slotmap~crates.io][c~slotmap~crates.io~badge]][c~slotmap~crates.io] [![slotmap~repo][c~slotmap~repo~badge]][c~slotmap~repo] [![slotmap~lib.rs][c~slotmap~lib.rs~badge]][c~slotmap~lib.rs]{{hi:slotmap}}{{hi:Arena}}{{hi:Allocator}}{{hi:Reference}}{{hi:slotmap}}{{hi:Storage}} [![cat~caching][cat~caching~badge]][cat~caching]{{hi:Caching}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}}

Use [`slotmap`][c~slotmap~docs]↗ to store collections of objects that need generated, unique, persistent identifiers ("keys"), but have no clear ownership otherwise, such as game entities or graph nodes.

`slotmap` items can be added or removed dynamically. Slotmap ensures stable indices, meaning once an item is inserted, its key remains valid until the item is explicitly removed.

[`slotmap`][c~slotmap~docs]↗ provides three containers: [`SlotMap`][c~slotmap~docs]↗, `HopSlotMap` and `DenseSlotMap`. Two secondary maps, `SecondaryMap` and [`SparseSecondaryMap`][c~slotmap::SparseSecondaryMap~docs]↗{{hi:SparseSecondaryMap}} are also provided to map additional objects to the keys created by one of the slot maps.

This example showcases how to create, insert, access, and remove elements from a `SlotMap`, and how to use a `SecondaryMap` to associate additional data with the elements in the `SlotMap`:

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/maps/slotmap.rs:example}}
```

## Remember Insertion Order with `linked-hash-map` {#linked-hash-map}

[![linked-hash-map][c~linked-hash-map~docs~badge]][c~linked-hash-map~docs]{{hi:linked-hash-map}} [![linked-hash-map~crates.io][c~linked-hash-map~crates.io~badge]][c~linked-hash-map~crates.io] [![linked-hash-map~repo][c~linked-hash-map~repo~badge]][c~linked-hash-map~repo] [![linked-hash-map~lib.rs][c~linked-hash-map~lib.rs~badge]][c~linked-hash-map~lib.rs] [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]

[`linked-hash-map`][c~linked-hash-map~docs]↗{{hi:linked-hash-map}} is a hash map that iterates in insertion order. It is older than [`indexmap`](#indexmap) and has not changed much recently: prefer `indexmap` for new code, and use `linked-hash-map` when you need its `LinkedHashMap` API, for example to move an entry to the end when it is used again.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/maps/linked_hash_map.rs:example}}
```

## Map Ranges of Keys to Values with `rangemap` {#rangemap}

[![rangemap][c~rangemap~docs~badge]][c~rangemap~docs]{{hi:rangemap}} [![rangemap~crates.io][c~rangemap~crates.io~badge]][c~rangemap~crates.io] [![rangemap~repo][c~rangemap~repo~badge]][c~rangemap~repo] [![rangemap~lib.rs][c~rangemap~lib.rs~badge]][c~rangemap~lib.rs] [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]

[`rangemap`][c~rangemap~docs]↗{{hi:rangemap}} stores values against ranges of keys instead of single keys, and merges or splits ranges as you insert. Range ends are exclusive, so `0..50` and `50..100` are adjacent and `100` is not covered. It suits address spaces, schedules and version ranges.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/rangemap/rangemap.rs:example}}
```

## Keep a Sorted Map in a Skip List with `skiplist` {#skiplist}

[![skiplist][c~skiplist~docs~badge]][c~skiplist~docs]{{hi:skiplist}} [![skiplist~crates.io][c~skiplist~crates.io~badge]][c~skiplist~crates.io] [![skiplist~repo][c~skiplist~repo~badge]][c~skiplist~repo] [![skiplist~lib.rs][c~skiplist~lib.rs~badge]][c~skiplist~lib.rs] [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]

[`skiplist`][c~skiplist~docs]↗{{hi:skiplist}} provides `SkipMap`, an ordered map built on a skip list. Iteration visits the keys in sorted order regardless of insertion order. For most programs, [`BTreeMap`][c~std::collections::BTreeMap~docs]↗ from the standard library is the simpler choice; a skip list is mainly of interest for its structure and for `skiplist`'s ordered-collection API.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/skiplist/skiplist.rs:example}}
```

## Test Membership Probabilistically with `bloomfilter` {#bloomfilter}

[![bloomfilter][c~bloomfilter~docs~badge]][c~bloomfilter~docs]{{hi:bloomfilter}} [![bloomfilter~crates.io][c~bloomfilter~crates.io~badge]][c~bloomfilter~crates.io] [![bloomfilter~repo][c~bloomfilter~repo~badge]][c~bloomfilter~repo] [![bloomfilter~lib.rs][c~bloomfilter~lib.rs~badge]][c~bloomfilter~lib.rs] [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]

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
