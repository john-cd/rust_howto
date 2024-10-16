# Data Structures

Rust implementations of ways of organizing data suited for specific purposes.

## Slotmap

[slotmap][c-slotmap-crates.io]⮳

`slotmpa` provides three containers with persistent unique keys to access stored values, `SlotMap`, `HopSlotMap` and `DenseSlotMap`. Upon insertion a key is returned that can be used to later access or remove the values. Insertion, deletion and access all take O(1) time with low overhead. Use it to store collections of objects that need stable, safe references but have no clear ownership otherwise, such as game entities or graph nodes. Two secondary maps, `SecondaryMap` and `SparseSecondaryMap` are also provided that map further objects to the keys created by one of the slot maps.

## See also

[Splay_tree][wikipedia-splay-tree]⮳

{{#include bitfield.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: expand
</div>
