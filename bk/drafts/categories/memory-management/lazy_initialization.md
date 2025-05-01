# Global Statics and Lazy Initialization

{{#include lazy_initialization.incl.md}}

- Immutable Global: use the `static` keyword (compile-time init, limited).
- Lazy Init: [`lazy_static`][c-lazy_static]⮳{{hi:lazy_static}} (runtime init, simple), [`once_cell`][c-once_cell]⮳{{hi:once_cell}} (runtime init, more control).
- Mutable Global: [`parking_lot::Mutex`][c-parking_lot::Mutex]⮳{{hi:parking_lot::Mutex}}/`RwLock` (thread-safe).
- Thread-Local: `std::thread_local`.
- Atomics: [`std::sync::atomic`][c-std::sync::atomic]⮳{{hi:std::sync::atomic}}.

Two key libraries:

- [`once_cell`][c-once_cell]⮳{{hi:once_cell}}: newer crate with more ergonomic API. Should be preferred for all new projects.
- [`lazy_static`][c-lazy_static]⮳{{hi:lazy_static}}: older crate. Its API is less convenient, but crate is stable and maintained.

Prefer [`once_cell`][c-once_cell]⮳{{hi:once_cell}} over lazy_static. Use mutexes / rwlocks for mutable globals. Consider alternatives to globals.

The core functionality of [`once_cell`][c-once_cell]⮳{{hi:once_cell}} is now included in the standard library with the remaining parts on track to be stabilized in future.

## `std` {#std}

[![std][c-std-badge]][c-std]{{hi:std}}
[![cat-memory-management][cat-memory-management-badge]][cat-memory-management]{{hi:Memory management}}

[`OnceCell`][c-std-core::cell::OnceCell]{{hi:OnceCell}}⮳ is a cell which can be written to only once.

The corresponding `Sync` version of `OnceCell<T>` is `OnceLock<T>`.

```rust,editable
{{#include ../../../crates/cats/memory_management/tests/once_cell/once_cell.rs:example}}
```

## `once_cell` {#once-cell}

[![once_cell][c-once_cell-badge]][c-once_cell]{{hi:once_cell}}
[![once_cell-crates.io][c-once_cell-crates.io-badge]][c-once_cell-crates.io]
[![once_cell-github][c-once_cell-github-badge]][c-once_cell-github]
[![once_cell-lib.rs][c-once_cell-lib.rs-badge]][c-once_cell-lib.rs]
[![cat-memory-management][cat-memory-management-badge]][cat-memory-management]{{hi:Memory management}}
[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

[`once_cell`][c-once_cell]{{hi:once_cell}}⮳ provides two cell-like types, `unsync::OnceCell` and `sync::OnceCell`. A `OnceCell` might store arbitrary non-Copy types, can be assigned to at most once and provides direct access to the stored contents. The `sync` flavor is thread-safe. [`once_cell`][c-once_cell]⮳{{hi:once_cell}} also has a [`once_cell::sync::Lazy`][c-once_cell::sync::Lazy]{{hi:once_cell::sync::Lazy}}⮳ type, build on top of [`OnceCell`][c-once_cell::sync::OnceCell]⮳:

```rust,editable
{{#include ../../../crates/cats/memory_management/tests/once_cell/once_cell2.rs:example}}
```

## `lazy_static` {#lazy-static}

[![lazy_static][c-lazy_static-badge]][c-lazy_static]{{hi:lazy_static}}
[![lazy_static-crates.io][c-lazy_static-crates.io-badge]][c-lazy_static-crates.io]
[![lazy_static-github][c-lazy_static-github-badge]][c-lazy_static-github]
[![lazy_static-lib.rs][c-lazy_static-lib.rs-badge]][c-lazy_static-lib.rs]
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}
[![cat-memory-management][cat-memory-management-badge]][cat-memory-management]{{hi:Memory management}}
[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

[`lazy_static`][c-lazy_static]⮳{{hi:lazy_static}}

```rust,editable
{{#include ../../../crates/cats/memory_management/tests/lazy_static/lazy_static.rs:example}}
```

## Declare Lazily Evaluated Constants {#declare-lazily-evaluated-constant}

[![lazy_static][c-lazy_static-badge]][c-lazy_static]{{hi:lazy_static}}
[![lazy_static-crates.io][c-lazy_static-crates.io-badge]][c-lazy_static-crates.io]
[![lazy_static-github][c-lazy_static-github-badge]][c-lazy_static-github]
[![lazy_static-lib.rs][c-lazy_static-lib.rs-badge]][c-lazy_static-lib.rs]
[![cat-caching][cat-caching-badge]][cat-caching]{{hi:Caching}}
[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}
[![cat-memory-management][cat-memory-management-badge]][cat-memory-management]{{hi:Memory management}}

Declares a lazily evaluated constant [`std::collections::HashMap`][c-std::collections::HashMap]{{hi:std::collections::HashMap}}⮳. The [`std::collections::HashMap`][c-std::collections::HashMap]{{hi:std::collections::HashMap}}⮳ will be evaluated once and stored behind a global static reference.

```rust,editable
{{#include ../../../crates/cats/memory_management/tests/lazy_static/lazy_constant.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[lazy_initialization: write / fix NOW](https://github.com/john-cd/rust_howto/issues/411)
</div>
