# Lazy Initialization

{{#include lazy_initialization.incl.md}}

[![std][std-badge]][std]  [![cat-memory-management][cat-memory-management-badge]][cat-memory-management]

[OnceCell][std-core::cell::OnceCell]⮳ is a cell which can be written to only once.

The corresponding Sync version of `OnceCell<T>` is `OnceLock<T>`.

```rust,editable
{{#include ../../../deps/tests/once_cell.rs}}
```

## Older library

[![once-cell][once-cell-badge]][once-cell]  [(lib.rs)][once-cell-librs]⮳  [![cat-memory-management][cat-memory-management-badge]][cat-memory-management]

[`once_cell`][once_cell]⮳ provides two cell-like types, `unsync::OnceCell` and `sync::OnceCell`. A OnceCell might store arbitrary non-Copy types, can be assigned to at most once and provides direct access to the stored contents. The `sync` flavor is thread-safe.

once_cell also has a [`Lazy<T>`][once_cell::sync::Lazy]⮳ type, build on top of [`OnceCell`][once_cell::sync::OnceCell]⮳:

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/once_cell2.rs}}
```

## See also

[![cat-memory-management][cat-memory-management-badge]][cat-memory-management]

[![lazy-static][lazy-static-badge]][lazy-static]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
