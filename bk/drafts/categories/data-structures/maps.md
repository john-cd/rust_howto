# Hashmap's Friends

{{#include maps.incl.md}}

## Store Data in an Insertion-ordered Map {#indexmap}

[![indexmap][c~indexmap~docs~badge]][c~indexmap~docs]{{hi:indexmap}}
[![indexmap~crates.io][c~indexmap~crates.io~badge]][c~indexmap~crates.io]
[![indexmap~github][c~indexmap~github~badge]][c~indexmap~github]
[![indexmap~lib.rs][c~indexmap~lib.rs~badge]][c~indexmap~lib.rs]
[![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

IndexMap is a data structure in Rust that combines the features of a hashmap and a vector.

[`indexmap`][c~indexmap~docs]⮳{{hi:indexmap}} offers a hash map that separately keeps track of insertion order and allows you to efficiently iterate over its elements in that order.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/maps/indexmap.rs:example}}
```

## Store Data in a `multimap` {#multimap}

[![multimap][c~multimap~docs~badge]][c~multimap~docs]{{hi:multimap}}
[![multimap~crates.io][c~multimap~crates.io~badge]][c~multimap~crates.io]
[![multimap~github][c~multimap~github~badge]][c~multimap~github]
[![multimap~lib.rs][c~multimap~lib.rs~badge]][c~multimap~lib.rs]

A MultiMap allows you to store multiple values for a single key, which can be useful when you need to associate several items with the same identifier.

[`multimap`][c~multimap~docs]⮳{{hi:multimap}} is implemented as a thin wrapper around [`std::collections::HashMap`][c~std::collections::HashMap~docs]⮳{{hi:std::collections::HashMap}}.

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/maps/multimap.rs:example}}
```

## Store Collections of Objects that Need Stable, Safe References {#slotmap}

[![slotmap][c~slotmap~docs~badge]][c~slotmap~docs]{{hi:slotmap}}
[![slotmap~crates.io][c~slotmap~crates.io~badge]][c~slotmap~crates.io]
[![slotmap~github][c~slotmap~github~badge]][c~slotmap~github]
[![slotmap~lib.rs][c~slotmap~lib.rs~badge]][c~slotmap~lib.rs]

Slotmap offers a way to handle collections where items can be added or removed dynamically, and each item is identified by a unique key. Slotmap ensures stable indices, meaning once an item is inserted, its key remains valid until the item is explicitly removed.

Use [`slotmap`][c~slotmap~docs]⮳{{hi:slotmap}} to store collections of objects that need stable, safe references but have no clear ownership otherwise, such as game entities or graph nodes.

[`slotmap`][c~slotmap~docs]⮳{{hi:slotmap}} provides three containers with persistent unique keys to access stored values, [`SlotMap`][c~slotmap~docs]⮳{{hi:SlotMap}}, `HopSlotMap` and `DenseSlotMap`. Two secondary maps, `SecondaryMap` and [`SparseSecondaryMap`][c~slotmap::SparseSecondaryMap~docs]⮳{{hi:SparseSecondaryMap}} are also provided that map further objects to the keys created by one of the slot maps.

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/maps/slotmap.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[maps: write / expand](https://github.com/john-cd/rust_howto/issues/281)
</div>
