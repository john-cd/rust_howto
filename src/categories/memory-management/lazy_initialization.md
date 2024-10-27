# Lazy Initialization

{{#include lazy_initialization.incl.md}}

[![std][c-std-badge]][c-std]{{hi:std}}  [![cat-memory-management][cat-memory-management-badge]][cat-memory-management]

[OnceCell][c-std-core::cell::OnceCell]⮳ is a cell which can be written to only once.

The corresponding Sync version of `OnceCell<T>` is `OnceLock<T>`.

```rust
{{#include ../../../deps/tests/once_cell.rs}}
```

## Older library

[![once_cell][c-once_cell-badge]][c-once_cell]{{hi:once_cell}}  [![once_cell-lib.rs][c-once_cell-lib.rs-badge]][c-once_cell-lib.rs]⮳  [![cat-memory-management][cat-memory-management-badge]][cat-memory-management]

[`once_cell`][c-once_cell]{{hi:once_cell}}⮳ provides two cell-like types, `unsync::OnceCell` and `sync::OnceCell`. A `OnceCell` might store arbitrary non-Copy types, can be assigned to at most once and provides direct access to the stored contents. The `sync` flavor is thread-safe. `once_cell` also has a [`once_cell::sync::Lazy`][c-once_cell::sync::Lazy]{{hi:once_cell::sync::Lazy}}⮳ type, build on top of [`OnceCell`][c-once_cell::sync::OnceCell]⮳:

```rust,mdbook-runnable
{{#include ../../../deps/tests/once_cell2.rs}}
```

## See also

[![cat-memory-management][cat-memory-management-badge]][cat-memory-management]

[![lazy_static][c-lazy_static-badge]][c-lazy_static]{{hi:lazy_static}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: review

The core functionality of once_cell is now included in the standard library with the remaining parts on track to be stabilised in future.

once_cell
Newer crate with more ergonomic API. Should be preferred for all new projects.

lazy_static
Older crate. API is less convenient, but crate is stable and maintained.
</div>
