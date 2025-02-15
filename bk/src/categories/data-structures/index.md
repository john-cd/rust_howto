# Data Structures

[![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}

Rust implementations of ways of organizing data suited for specific purposes.

## Bitflags

{{#include bitfield.incl.md}}

## Hashmaps and friends

{{#include maps.incl.md}}

## Stack-allocated arrays

{{#include stack_allocated_arrays.incl.md}}

## UUIDs

{{#include uuid.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[data-structures: expand (P1)](https://github.com/john-cd/rust_howto/issues/280)

`Hashset`
`BinaryHeap`
`LinkedList`
`Stack`
`Queue`
`BTreeMap`
`BTreeSet`

Alongside [`arrayvec`][c-arrayvec]⮳{{hi:arrayvec}} and [`tinyvec`][c-tinyvec]⮳{{hi:tinyvec}}, `heapless` has stack-allocated arrays, but also includes:

Arc – like [`std::sync::Arc`][c-std::sync::Arc]⮳{{hi:std::sync::Arc}} but backed by a lock-free memory pool rather than `#[global_allocator]`
Box – like [`std::boxed::Box`][c-std::boxed::Box]⮳{{hi:std::boxed::Box}} but backed by a lock-free memory pool rather than `#[global_allocator]`
`BinaryHeap` – priority queue
[`IndexMap`][c-indexmap]⮳{{hi:IndexMap}} – hash table
`IndexSet` – hash set
`LinearMap`
`Object`{{hi:Object}}  – objects managed by an object pool
[`String`][c-std::string::String]⮳{{hi:String}}
`Vec`
mpmc::Q* – multiple producer multiple consumer lock-free queue
`spsc::Queue` – single producer single consumer lock-free queue
</div>
