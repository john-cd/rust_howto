# Hashmap's friends

{{#include maps.incl.md}}

## Insertion-ordered map {#indexmap}

[![indexmap][c-indexmap-badge]][c-indexmap]{{hi:indexmap}}
[![indexmap-crates.io][c-indexmap-crates.io-badge]][c-indexmap-crates.io]
[![indexmap-github][c-indexmap-github-badge]][c-indexmap-github]
[![indexmap-lib.rs][c-indexmap-lib.rs-badge]][c-indexmap-lib.rs]
[![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

A HashMap that seperately keeps track of insertion order and allows you to efficiently iterate over its elements in that order

## Multimap {#multimap}

[![multimap][c-multimap-badge]][c-multimap]{{hi:multimap}}
[![multimap-crates.io][c-multimap-crates.io-badge]][c-multimap-crates.io]
[![multimap-github][c-multimap-github-badge]][c-multimap-github]
[![multimap-lib.rs][c-multimap-lib.rs-badge]][c-multimap-lib.rs]

`multimap` is implemented as a thin wrapper around `std::collections::HashMap`. It allows multiple values for a given key.

```rust,editable
{{#include ../../../deps/tests/cats/data_structures/multimap.rs:example}}
```

## Slotmap {#slotmap}

[![slotmap][c-slotmap-badge]][c-slotmap]{{hi:slotmap}}
[![slotmap-crates.io][c-slotmap-crates.io-badge]][c-slotmap-crates.io]
[![slotmap-github][c-slotmap-github-badge]][c-slotmap-github]
[![slotmap-lib.rs][c-slotmap-lib.rs-badge]][c-slotmap-lib.rs]

Use to store collections of objects that need stable, safe references but have no clear ownership otherwise, such as game entities or graph nodes.

`slotmap` provides three containers with persistent unique keys to access stored values, `SlotMap` , `HopSlotMap` and `DenseSlotMap`. Upon insertion a key is returned that can be used to later access or remove the values. Insertion, deletion and access all take O(1) time with low overhead.  Two secondary maps, `SecondaryMap` and `SparseSecondaryMap` are also provided that map further objects to the keys created by one of the slot maps.

## See also

[Splay tree][wikipedia-splay-tree]{{hi:Splay tree}}⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write / expand
</div>
