# Hashmap's friends

{{#include maps.incl.md}}

## Store data in an insertion-ordered map {#indexmap}

[![indexmap][c-indexmap-badge]][c-indexmap]{{hi:indexmap}}
[![indexmap-crates.io][c-indexmap-crates.io-badge]][c-indexmap-crates.io]
[![indexmap-github][c-indexmap-github-badge]][c-indexmap-github]
[![indexmap-lib.rs][c-indexmap-lib.rs-badge]][c-indexmap-lib.rs]
[![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

`indexmap` offers a hash map that separately keeps track of insertion order and allows you to efficiently iterate over its elements in that order.

```rust,editable
{{#include ../../../crates/ex/cats/data_structures/tests/indexmap.rs:example}}
```

## Store data in a multimap {#multimap}

[![multimap][c-multimap-badge]][c-multimap]{{hi:multimap}}
[![multimap-crates.io][c-multimap-crates.io-badge]][c-multimap-crates.io]
[![multimap-github][c-multimap-github-badge]][c-multimap-github]
[![multimap-lib.rs][c-multimap-lib.rs-badge]][c-multimap-lib.rs]

`multimap` is implemented as a thin wrapper around `std::collections::HashMap`. It allows multiple values for a given key.

```rust,editable,noplayground
{{#include ../../../crates/ex/cats/data_structures/tests/multimap.rs:example}}
```

## Store collections of objects that need stable, safe references {#slotmap}

[![slotmap][c-slotmap-badge]][c-slotmap]{{hi:slotmap}}
[![slotmap-crates.io][c-slotmap-crates.io-badge]][c-slotmap-crates.io]
[![slotmap-github][c-slotmap-github-badge]][c-slotmap-github]
[![slotmap-lib.rs][c-slotmap-lib.rs-badge]][c-slotmap-lib.rs]

Use `slotmap` to store collections of objects that need stable, safe references but have no clear ownership otherwise, such as game entities or graph nodes.

`slotmap` provides three containers with persistent unique keys to access stored values, `SlotMap` , `HopSlotMap` and `DenseSlotMap`. Two secondary maps, `SecondaryMap` and `SparseSecondaryMap` are also provided that map further objects to the keys created by one of the slot maps.

```rust,editable,noplayground
{{#include ../../../crates/ex/cats/data_structures/tests/slotmap.rs:example}}
```

## See also

[Splay tree][wikipedia-splay-tree]{{hi:Splay tree}}⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[maps: write / expand (P1)](https://github.com/john-cd/rust_howto/issues/281)
</div>
