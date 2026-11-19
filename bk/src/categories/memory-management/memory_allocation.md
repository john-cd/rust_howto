<!-- TODO Reviewed ??-??-2026. -->

# Custom Memory Allocation

{{#include memory_allocation.incl.md}}

## Custom Memory Allocators {#custom-memory-allocators .skip}

### Use a Custom Allocator with `tikv-jemallocator` {#tikv-jemallocator}

{{#badges tikv-jemallocator memory-management="Memory management" api-bindings="API bindings"}}{{hi:Allocator}}{{hi:Jemalloc}}

`tikv-jemallocator` is a Rust wrapper around jemalloc, a widely used allocator written in C. It can serve as the process-wide allocator for Rust allocations.

Test this Cargo-backed example from `bk/crates` with `cargo test --locked -p memory_management --example jemalloc`.

```rust,editable,noplayground
{{#include ../../../crates/cats/memory_management/examples/jemalloc.rs:example}}
```

### Use the `mimalloc` Memory Allocator {#mimalloc}

{{#badges mimalloc api-bindings="API bindings" memory-management="Memory management"}}{{hi:Performance}}{{hi:Allocator}}{{hi:mimalloc}}

Mimalloc is a general-purpose, performance-oriented allocator developed by Microsoft. It can serve as the process-wide allocator for Rust allocations.

Test this Cargo-backed example from `bk/crates` with `cargo test --locked -p memory_management --example mimalloc`.

```rust,editable,noplayground
{{#include ../../../crates/cats/memory_management/examples/mimalloc.rs:example}}
```

### Select the Global Allocator with Cargo Features {#global-allocator}

[![mimalloc][c~mimalloc~docs~badge]][c~mimalloc~docs]{{hi:mimalloc}} [![tikv-jemallocator][c~tikv-jemallocator~docs~badge]][c~tikv-jemallocator~docs]{{hi:tikv-jemallocator}}{{hi:Global allocator}}{{hi:Cargo features}} [![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}}

A library or an application can let the build choose the [`#[global_allocator]`][c~std::alloc::GlobalAlloc~docs]↗{{hi:GlobalAlloc}}. The example defines the features `use-mimalloc` and `use-jemalloc`, rejects builds that enable both features, and falls back to the system allocator when neither is. It lives in a separate package so its mutually exclusive features do not conflict with the main workspace's `--all-features` checks. From `bk/`, run `just cargo run --manifest-path crates/cats/memory_management/allocator_selection/Cargo.toml --features use-mimalloc`. On MSVC, selecting jemalloc falls back to the system allocator.

```rust,editable,noplayground
{{#include ../../../crates/cats/memory_management/allocator_selection/src/global_allocator.rs:example}}
```

Measure before switching: the best allocator depends on the workload and the platform. `tikv-jemallocator` does not support the MSVC target.

## Pools and Arenas {#storage-strategies .skip}

### `slab` {#slab}

{{#badges slab data-structures="Data structures" memory-management="Memory management" no-std="No standard library"}}{{hi:slab}}{{hi:Allocator}}{{hi:No_std}}

[`Slab`][c~slab~docs]↗{{hi:Slab}} stores values in indexed slots and reuses a slot after its value is removed. It grows as needed; use `with_capacity` to reserve slots up front. Its integer keys suit pools of similar values, but a removed key may be reused. Choose generational keys when stale handles must be rejected.

```rust,editable
{{#include ../../../crates/cats/memory_management/examples/slab.rs:example}}
```

### `bumpalo` {#bumpalo}

{{#badges bumpalo memory-management="Memory management" no-std="No standard library" rust-patterns="Rust patterns"}}

[`bumpalo`][c~bumpalo~docs]↗{{hi:bumpalo}} is a fast bump allocation arena for Rust. Bump allocation works by maintaining a pointer to a region of memory; each allocation simply advances this pointer, making allocations extremely cheap (O(1)). All memory is freed at once when the arena is dropped, which makes it ideal for allocating many short-lived objects with the same lifetime.

```rust,editable
{{#include ../../../crates/cats/memory_management/examples/bumpalo.rs:example}}
```

## Concurrent Memory Reclamation with `seize` {#seize}

{{#badges seize concurrency=Concurrency memory-management="Memory management"}}{{hi:Memory reclamation}}{{hi:Concurrency}}{{hi:Lock-free}}

[`seize`][c~seize~docs]↗{{hi:seize}} implements deferred memory reclamation for concurrent data structures. A collector tracks active guards, and `retire` delays dropping an object until no protected reader can still access it. The example shows the unpublish-and-retire sequence; it is an API sketch, not a complete concurrent data structure.

The key types are:

- `Collector`: tracks active readers and coordinates deferred reclamation.
- `Guard` (returned by `collector.enter()`): keeps protected objects from being reclaimed while the guard is held.
- `collector.retire(ptr, reclaim::boxed)`: schedules a pointer for deferred reclamation once no threads hold a guard that could have seen it.

Test this Cargo-backed example from `bk/crates` with `cargo test --locked -p memory_management --example seize`.

```rust,editable,noplayground
{{#include ../../../crates/cats/memory_management/examples/seize.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Memory Management overview](index.md)
- [Arena Allocation](arena_allocation.md)
- [String Interning](string_interning.md)
- [Global Statics and Lazy Initialization](lazy_initialization.md)
- [Smart Pointers](../../standard-library/smart_pointers.md)
- [Development Tools - Profiling](../development-tools_profiling/index.md)

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

- [Jemalloc][c~jemalloc~lib.rs]↗.

</div>
