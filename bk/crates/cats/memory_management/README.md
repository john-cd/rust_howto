<!-- TODO Reviewed ??-??-2026 -->

# Memory Management

Examples of advanced memory management, allocators, and data structures like Slab and Bumpalo. It demonstrates integration with key ecosystem crates including `anyhow`, `bumpalo`, `clap`, `lazy_static`, `mimalloc`, `once_cell`.

## Examples Covered

- **Lazy Initialization** (`examples/lazy_initialization/`): Demonstrates concepts like `lazy_cell`, `lazy_constant`, `lazy_lock`, `lazy_static`, `once_cell`, `once_cell2`, `once_lock`.
- **Standalone Examples**: `bumpalo`, `bumpalo_tree`, `global_allocator`, `interning`, `interner_allocations`, `jemalloc`, `mimalloc`, `seize`, `slab`.

## Usage

To check or run examples in this crate:

```console
cargo check --package memory_management --examples
cargo test --package memory_management
```

Code examples correspond to chapters in `bk/src/categories/memory_management` or related book sections.

## NOTES

A few tests use attributes that must be applied at the crate level.
As such, they must be placed at the root of the `tests` folder.
