# Lazy Init

[![std-badge]][std]

[OnceCell][std-core::cell::OnceCell]⮳ is a cell which can be written to only once.

The corresponding Sync version of `OnceCell<T>` is `OnceLock<T>`.

```rust,editable
{{#include ../../deps/examples/once_cell.rs}}
```

## Older library

[![once-cell-badge]][once-cell] [(lib.rs)][once-cell-librs]⮳

`once_cell` provides two cell-like types, `unsync::OnceCell` and `sync::OnceCell`. A OnceCell might store arbitrary non-Copy types, can be assigned to at most once and provides direct access to the stored contents. The `sync` flavor is thread-safe.

once_cell also has a `Lazy<T>` type, build on top of `OnceCell`:

```rust,editable,mdbook-runnable
{{#include ../../deps/examples/once_cell2.rs}}
```

## See also

[![lazy-static-badge]][lazy-static]

{{#include ../refs/link-refs.md}}
