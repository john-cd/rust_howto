# HashMap's Friends: `IndexMap`, `MultiMap`, and `SlotMap`

{{#include maps.incl.md}}

## Store Data in an Insertion-ordered Map {#indexmap}

[![indexmap][c~indexmap~docs~badge]][c~indexmap~docs] [![indexmap~crates.io][c~indexmap~crates.io~badge]][c~indexmap~crates.io] [![indexmap~repo][c~indexmap~repo~badge]][c~indexmap~repo] [![indexmap~lib.rs][c~indexmap~lib.rs~badge]][c~indexmap~lib.rs]{{hi:indexmap}}{{hi:Hashmap}}{{hi:No_std}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`indexmap`][c~indexmap~docs]↗{{hi:indexmap}} offers the `IndexMap` data structure that combines the features of a hashmap and a vector.
It is a hash table that keeps track of insertion order and allows efficiently iteration over its elements in that order.

This example demonstrates the usage of `IndexMap`, including methods for accessing elements by index and using the `entry` API:

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/maps/indexmap.rs:example}}
```

## Store Multiple Values for a Single Key with `multimap` {#multimap}

[![multimap][c~multimap~docs~badge]][c~multimap~docs] [![multimap~crates.io][c~multimap~crates.io~badge]][c~multimap~crates.io] [![multimap~repo][c~multimap~repo~badge]][c~multimap~repo] [![multimap~lib.rs][c~multimap~lib.rs~badge]][c~multimap~lib.rs]{{hi:multimap}}

A `MultiMap` allows storing multiple values for a single key, which can be useful to associate several items with the same identifier. [`multimap`][c~multimap~docs]↗ is implemented as a thin wrapper around [`std::collections::HashMap`][c~std::collections::HashMap~docs]↗{{hi:std::collections::HashMap}}.

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

## Related Topics {#related-topics .skip}

- [[graphs | Graphs]].
- [[hashmaps | Hashmaps]].
- [[vectors | Vectors]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
