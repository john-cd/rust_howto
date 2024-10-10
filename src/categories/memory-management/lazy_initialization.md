# Lazy Initialization

{{#include lazy_initialization.incl.md}}

[![std][c-std-badge]][c-std]  [![cat-memory-management][cat-memory-management-badge]][cat-memory-management]

[OnceCell][std-core::cell::OnceCell]⮳ is a cell which can be written to only once.

The corresponding Sync version of `OnceCell<T>` is `OnceLock<T>`.

```rust,editable
{{#include ../../../deps/tests/once_cell.rs}}
```

## Older library

[![once-cell][c-once-cell-badge]][c-once-cell]  [![once-cell-librs][c-once-cell-librs-badge]][c-once-cell-librs]⮳  [![cat-memory-management][cat-memory-management-badge]][cat-memory-management]

[`{{i:once_cell}}`][c-once-cell]⮳ provides two cell-like types, `unsync::OnceCell` and `sync::OnceCell`. A OnceCell might store arbitrary non-Copy types, can be assigned to at most once and provides direct access to the stored contents. The `sync` flavor is thread-safe. `once_cell` also has a [`{{i:Lazy}}<T>`][c-once_cell::sync::Lazy]⮳ type, build on top of [`{{i:OnceCell}}`][c-once_cell::sync::OnceCell]⮳:

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/once_cell2.rs}}
```

## See also

[![cat-memory-management][cat-memory-management-badge]][cat-memory-management]

[![lazy-static][c-lazy-static-badge]][c-lazy-static]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
