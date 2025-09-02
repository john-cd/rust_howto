# Custom Memory Allocation

{{#include memory_allocation.incl.md}}

## Custom Memory Allocators {#custom-memory-allocators .skip}

### Use a Custom Allocator `wee_alloc` {#wee_alloc}

[![wee_alloc][c~wee_alloc~docs~badge]][c~wee_alloc~docs] [![wee_alloc~crates.io][c~wee_alloc~crates.io~badge]][c~wee_alloc~crates.io] [![wee_alloc~repo][c~wee_alloc~repo~badge]][c~wee_alloc~repo] [![wee_alloc~lib.rs][c~wee_alloc~lib.rs~badge]][c~wee_alloc~lib.rs]{{hi:wee_alloc}} [![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~wasm][cat~wasm~badge]][cat~wasm]{{hi:WebAssembly}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}} [![cat~embedded][cat~embedded~badge]][cat~embedded]{{hi:Embedded development}}

[`wee_alloc`][c~wee_alloc~docs]↗{{hi:wee_alloc}} is a WASM-enabled allocator.

### Use a Custom Allocator with `tikv-jemallocator` {#tikv-jemallocator}

[![tikv-jemallocator][c~tikv-jemallocator~docs~badge]][c~tikv-jemallocator~docs] [![tikv-jemallocator~crates.io][c~tikv-jemallocator~crates.io~badge]][c~tikv-jemallocator~crates.io] [![tikv-jemallocator~repo][c~tikv-jemallocator~repo~badge]][c~tikv-jemallocator~repo] [![tikv-jemallocator~lib.rs][c~tikv-jemallocator~lib.rs~badge]][c~tikv-jemallocator~lib.rs]{{hi:tikv-jemallocator}}{{hi:Allocator}}{{hi:Jemalloc}} [![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}} [![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}}

`tikv-jemallocator` is a Rust allocator backed by 'jemalloc' (a well-known C library). It is a drop-in replacement for the standard Rust allocator (in `alloc::alloc`).

```rust,editable
{{#include ../../../crates/cats/memory_management/examples/jemalloc.rs:example}}
```

### Use the `mimalloc` Memory Allocator {#mimalloc}

[![mimalloc][c~mimalloc~docs~badge]][c~mimalloc~docs] [![mimalloc~crates.io][c~mimalloc~crates.io~badge]][c~mimalloc~crates.io] [![mimalloc~repo][c~mimalloc~repo~badge]][c~mimalloc~repo] [![mimalloc~lib.rs][c~mimalloc~lib.rs~badge]][c~mimalloc~lib.rs]{{hi:mimalloc}}{{hi:Performance}}{{hi:Allocator}}{{hi:mimalloc}}{{hi:Encrypted-heap}} [![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}} [![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}}

Mimalloc is a general purpose, performance-oriented allocator built by Microsoft. It is also a drop-in replacement for the standard Rust allocator (in `alloc::alloc`).

```rust,editable
{{#include ../../../crates/cats/memory_management/examples/mimalloc.rs:example}}
```

## Pre-allocated Storage for a Uniform Data Type {#pre-allocated-storage .skip}

### `slab` {#slab}

[![slab][c~slab~docs~badge]][c~slab~docs] [![slab~crates.io][c~slab~crates.io~badge]][c~slab~crates.io] [![slab~repo][c~slab~repo~badge]][c~slab~repo] [![slab~lib.rs][c~slab~lib.rs~badge]][c~slab~lib.rs]{{hi:slab}}{{hi:slab}}{{hi:Allocator}}{{hi:No_std}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`slab`][c~slab~docs]↗{{hi:slab}} provides pre-allocated storage for a single data type. If many values of a single type are being allocated, it can be more efficient to pre-allocate the necessary storage. Since the size of the type is uniform, memory fragmentation can be avoided. Storing, clearing, and lookup operations become very cheap.

While [`slab`][c~slab~docs]↗{{hi:slab}} may look like other Rust collections, it is not intended to be used as a general purpose collection. The primary difference between `slab` and `Vec` is that [`slab`][c~slab~docs]↗{{hi:slab}} returns the key when storing the value.

It is important to note that keys may be reused. In other words, once a value associated with a given key is removed from a slab, that key may be returned from future calls to insert.

TODO add example

### `bumpalo` {#bumpalo}

[![bumpalo][c~bumpalo~docs~badge]][c~bumpalo~docs] [![bumpalo~crates.io][c~bumpalo~crates.io~badge]][c~bumpalo~crates.io] [![bumpalo~repo][c~bumpalo~repo~badge]][c~bumpalo~repo] [![bumpalo~lib.rs][c~bumpalo~lib.rs~badge]][c~bumpalo~lib.rs]{{hi:bumpalo}} [![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

[`bumpalo`][c~bumpalo~docs]↗{{hi:bumpalo}} is a fast bump allocation arena for Rust.

TODO add example?

## Garbage Collection with `seize` {#seize}

[![seize][c~seize~docs~badge]][c~seize~docs] [![seize~crates.io][c~seize~crates.io~badge]][c~seize~crates.io] [![seize~repo][c~seize~repo~badge]][c~seize~repo] [![seize~lib.rs][c~seize~lib.rs~badge]][c~seize~lib.rs]{{hi:seize}}{{hi:Garbage}}{{hi:Concurrency}}{{hi:Rcu}}{{hi:Atomic}}{{hi:Lock-free}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}}

[`seize`][c~seize~docs]↗{{hi:seize}} allows fast, efficient, and predictable memory reclamation for concurrent data structures.

TODO add example?

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1341)

- [Jemalloc][c~jemalloc~lib.rs]↗.

</div>
