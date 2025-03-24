# Custom Memory Allocation

{{#include memory_allocation.incl.md}}

## `wee_alloc` {#wee_alloc}

[![wee_alloc][c-wee_alloc-badge]][c-wee_alloc] [![wee_alloc-crates.io][c-wee_alloc-crates.io-badge]][c-wee_alloc-crates.io] [![wee_alloc-github][c-wee_alloc-github-badge]][c-wee_alloc-github] [![wee_alloc-lib.rs][c-wee_alloc-lib.rs-badge]][c-wee_alloc-lib.rs]{{hi:wee_alloc}} [![cat-memory-management][cat-memory-management-badge]][cat-memory-management]{{hi:Memory management}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}} [![cat-wasm][cat-wasm-badge]][cat-wasm]{{hi:WebAssembly}} [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]{{hi:Web programming}} [![cat-embedded][cat-embedded-badge]][cat-embedded]{{hi:Embedded development}}

wee_alloc: The Wasm-Enabled, Elfin Allocator

## Pre-allocated Storage for a Uniform Data Type {#skip}

### `slab` {#slab}

[![slab][c-slab-badge]][c-slab] [![slab-crates.io][c-slab-crates.io-badge]][c-slab-crates.io] [![slab-github][c-slab-github-badge]][c-slab-github] [![slab-lib.rs][c-slab-lib.rs-badge]][c-slab-lib.rs]{{hi:slab}}{{hi:slab}}{{hi:Allocator}}{{hi:No_std}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}} [![cat-memory-management][cat-memory-management-badge]][cat-memory-management]{{hi:Memory management}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

`slab` provides pre-allocated storage for a single data type. If many values of a single type are being allocated, it can be more efficient to pre-allocate the necessary storage. Since the size of the type is uniform, memory fragmentation can be avoided. Storing, clearing, and lookup operations become very cheap.

While `slab` may look like other Rust collections, it is not intended to be used as a general purpose collection. The primary difference between `slab` and `Vec` is that `slab` returns the key when storing the value.

It is important to note that keys may be reused. In other words, once a value associated with a given key is removed from a slab, that key may be returned from future calls to insert.

### `bumpalo` {#bumpalo}

[![bumpalo][c-bumpalo-badge]][c-bumpalo] [![bumpalo-crates.io][c-bumpalo-crates.io-badge]][c-bumpalo-crates.io] [![bumpalo-github][c-bumpalo-github-badge]][c-bumpalo-github] [![bumpalo-lib.rs][c-bumpalo-lib.rs-badge]][c-bumpalo-lib.rs]{{hi:bumpalo}} [![cat-memory-management][cat-memory-management-badge]][cat-memory-management]{{hi:Memory management}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}} [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

A fast bump allocation arena for Rust.

## Garbage Collection with `seize` {#seize}

[![seize][c-seize-badge]][c-seize] [![seize-crates.io][c-seize-crates.io-badge]][c-seize-crates.io] [![seize-github][c-seize-github-badge]][c-seize-github] [![seize-lib.rs][c-seize-lib.rs-badge]][c-seize-lib.rs]{{hi:seize}}{{hi:Garbage}}{{hi:Concurrency}}{{hi:Rcu}}{{hi:Atomic}}{{hi:Lock-free}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}} [![cat-memory-management][cat-memory-management-badge]][cat-memory-management]{{hi:Memory management}}

Fast, efficient, and predictable memory reclamation for concurrent data structures.

[`seize`][c-seize]⮳{{hi:seize}} FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write NOW
other allocators
</div>
