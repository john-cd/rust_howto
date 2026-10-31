<!-- TODO Reviewed ??-??-2026 -->

# Custom Memory Allocation

{{#include memory_allocation.incl.md}}

## Custom Memory Allocators {#custom-memory-allocators .skip}

### Use a Custom Allocator `wee_alloc` {#wee_alloc}

{{#badges wee_alloc memory-management="Memory management" no-std="No standard library" wasm=WebAssembly web-programming="Web programming" embedded="Embedded development"}}

[`wee_alloc`][c~wee_alloc~docs]↗{{hi:wee_alloc}} is a WASM-enabled allocator.

### Use a Custom Allocator with `tikv-jemallocator` {#tikv-jemallocator}

{{#badges tikv-jemallocator memory-management="Memory management" api-bindings="API bindings"}}{{hi:Allocator}}{{hi:Jemalloc}}

`tikv-jemallocator` is a Rust allocator backed by 'jemalloc' (a well-known C library). It is a drop-in replacement for the standard Rust allocator (in `alloc::alloc`).

```rust,editable
{{#include ../../../crates/cats/memory_management/examples/jemalloc.rs:example}}
```

### Use the `mimalloc` Memory Allocator {#mimalloc}

{{#badges mimalloc api-bindings="API bindings" memory-management="Memory management"}}{{hi:Performance}}{{hi:Allocator}}{{hi:mimalloc}}{{hi:Encrypted-heap}}

Mimalloc is a general purpose, performance-oriented allocator built by Microsoft. It is also a drop-in replacement for the standard Rust allocator (in `alloc::alloc`).

```rust,editable
{{#include ../../../crates/cats/memory_management/examples/mimalloc.rs:example}}
```

### Select the Global Allocator with Cargo Features {#global-allocator}

[![mimalloc][c~mimalloc~docs~badge]][c~mimalloc~docs]{{hi:mimalloc}} [![tikv-jemallocator][c~tikv-jemallocator~docs~badge]][c~tikv-jemallocator~docs]{{hi:tikv-jemallocator}}{{hi:Global allocator}}{{hi:Cargo features}} [![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}}

A library or an application can let the build choose the [`#[global_allocator]`][c~std::alloc::GlobalAlloc~docs]↗{{hi:GlobalAlloc}}. The example defines the features `use-mimalloc` and `use-jemalloc`, rejects builds that enable both features, and falls back to the system allocator when neither is. It lives in a separate package so its mutually exclusive features do not conflict with the main workspace's `--all-features` checks. From `bk/`, run `just cargo run --manifest-path crates/cats/memory_management/allocator_selection/Cargo.toml --features use-mimalloc`. On MSVC, selecting jemalloc falls back to the system allocator.

```rust,editable,noplayground
{{#include ../../../crates/cats/memory_management/allocator_selection/src/main.rs:example}}
```

Measure before switching: the best allocator depends on the workload and the platform. `tikv-jemallocator` does not support the MSVC target.

## Pre-allocated Storage for a Uniform Data Type {#pre-allocated-storage .skip}

### `slab` {#slab}

{{#badges slab data-structures="Data structures" memory-management="Memory management" no-std="No standard library"}}{{hi:slab}}{{hi:Allocator}}{{hi:No_std}}

[`slab`][c~slab~docs]↗{{hi:slab}} provides pre-allocated storage for a single data type. If many values of a single type are being allocated, it can be more efficient to pre-allocate the necessary storage. Since the size of the type is uniform, memory fragmentation can be avoided. Storing, clearing, and lookup operations become very cheap.

While [`slab`][c~slab~docs]↗{{hi:slab}} may look like other Rust collections, it is not intended to be used as a general purpose collection. The primary difference between `slab` and `Vec` is that [`slab`][c~slab~docs]↗{{hi:slab}} returns the key when storing the value.

It is important to note that keys may be reused. In other words, once a value associated with a given key is removed from a slab, that key may be returned from future calls to insert.

```rust,editable
{{#include ../../../crates/cats/memory_management/examples/slab.rs:example}}
```

### `bumpalo` {#bumpalo}

{{#badges bumpalo memory-management="Memory management" no-std="No standard library" rust-patterns="Rust patterns"}}

[`bumpalo`][c~bumpalo~docs]↗{{hi:bumpalo}} is a fast bump allocation arena for Rust. Bump allocation works by maintaining a pointer to a region of memory; each allocation simply advances this pointer, making allocations extremely cheap (O(1)). All memory is freed at once when the arena is dropped, which makes it ideal for allocating many short-lived objects with the same lifetime.

```rust,editable
{{#include ../../../crates/cats/memory_management/examples/bumpalo.rs:example}}
```

## Garbage Collection with `seize` {#seize}

{{#badges seize concurrency=Concurrency memory-management="Memory management"}}{{hi:Garbage}}{{hi:Concurrency}}{{hi:Rcu}}{{hi:Atomic}}{{hi:Lock-free}}

[`seize`][c~seize~docs]↗{{hi:seize}} allows fast, efficient, and predictable memory reclamation for concurrent data structures. It solves the problem of safely freeing shared objects when multiple threads may be reading them simultaneously, without using locks.

The key types are:

- `Collector`: manages epochs and tracks which threads are active.
- `Guard` (returned by `collector.enter()` / `Collector::enter`): while held, prevents reclamation of any objects loaded through it.
- `collector.retire(ptr, reclaim::boxed)`: schedules a pointer for deferred reclamation once no threads hold a guard that could have seen it.

```rust,editable
{{#include ../../../crates/cats/memory_management/examples/seize.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[memory-management/index | Memory Management overview]]
- [[arena_allocation | Arena Allocation]]
- [[string_interning | String Interning]]
- [[lazy_initialization | Global Statics and Lazy Initialization]]
- [[smart_pointers | Smart Pointers]]
- [[shared_state | Shared State]]
- [[development-tools_profiling | Development Tools: Profiling]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

- [Jemalloc][c~jemalloc~lib.rs]↗.

</div>
