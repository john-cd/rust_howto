# Global Statics and Lazy Initialization

{{#include lazy_initialization.incl.md}}

- Immutable Global: use the [`static`][keyword~static]↗{{hi:static}} keyword (compile-time init, limited).
- Lazy Init: [`lazy_static`][c~lazy_static~docs]↗{{hi:lazy_static}} (runtime init, simple), [`once_cell`][c~once_cell~docs]↗{{hi:once_cell}} (runtime init, more control).
- Mutable Global: [`parking_lot::Mutex`][c~parking_lot::Mutex~docs]↗{{hi:parking_lot::Mutex}}/[`RwLock`][c~std::sync::RwLock~docs]↗{{hi:RwLock}} (thread-safe).
- Thread-Local: `std::thread_local`.
- Atomics: [`std::sync::atomic`][c~std::sync::atomic~docs]↗{{hi:std::sync::atomic}}.

Two key libraries:

- [`once_cell`][c~once_cell~docs]↗{{hi:once_cell}}: newer crate with more ergonomic API. Should be preferred for all new projects.
- [`lazy_static`][c~lazy_static~docs]↗{{hi:lazy_static}}: older crate. Its API is less convenient, but crate is stable and maintained.

Prefer [`once_cell`][c~once_cell~docs]↗{{hi:once_cell}} over lazy_static. Use mutexes / rwlocks for mutable globals. Consider alternatives to globals.

The core functionality of [`once_cell`][c~once_cell~docs]↗{{hi:once_cell}} is now included in the standard library with the remaining parts on track to be stabilized in future.

## Lazy Initialize Variables {#std}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}
[![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}}

For lazy initialization scenarios where you want to defer the creation of a value until it's actually needed, without the overhead of thread synchronization, use one of the following:

- The [`OnceCell`][c~core::cell::OnceCell~docs]↗{{hi:OnceCell}} type provides a way to define a value that will be initialized at most once.
- The corresponding thread-safe version of `OnceCell<T>` is `OnceLock<T>`. [`OnceLock<T>`][c~std::sync::OnceLock~docs]↗ is a lock that allows a value to be initialized exactly once, ensuring that the initialization code is executed only once, even in the presence of multiple threads.
- [`LazyCell<T>`][c~std::cell::LazyCell~docs]↗ is a value which is initialized on the first access. It is not thread-safe.
- [`LazyLock`][c~std::sync::LazyLock~docs]↗ is a lazily initialized value that is initialized on first access, ensuring that the initialization code is executed only once, even in the presence of multiple threads. This type is a thread-safe [`LazyCell`][c~std::cell::LazyCell~docs]↗{{hi:LazyCell}}, and can be used in statics. Since initialization may be called from multiple threads, any dereferencing call will block the calling thread if another initialization routine is currently running.
- [`Once`][c~std::sync::Once~docs]↗ is a low-level primitive that allows initialization of a value exactly once, ensuring that the initialization code is executed only once, even in the presence of multiple threads.

```rust,editable
{{#include ../../../crates/cats/memory_management/examples/lazy_initialization/once_cell.rs:example}}
```

## `once_cell` {#once-cell}

[![once_cell][c~once_cell~docs~badge]][c~once_cell~docs]{{hi:once_cell}}
[![once_cell~crates.io][c~once_cell~crates.io~badge]][c~once_cell~crates.io]
[![once_cell~github][c~once_cell~github~badge]][c~once_cell~github]
[![once_cell~lib.rs][c~once_cell~lib.rs~badge]][c~once_cell~lib.rs]
[![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}}
[![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

[`once_cell`][c~once_cell~docs]↗{{hi:once_cell}} provides two cell-like types, `unsync::OnceCell` and `sync::OnceCell`. A `OnceCell` might store arbitrary non-Copy types, can be assigned to at most once and provides direct access to the stored contents. The `sync` flavor is thread-safe. [`once_cell`][c~once_cell~docs]↗{{hi:once_cell}} also has a [`once_cell::sync::Lazy`][c~once_cell::sync::Lazy~docs]↗{{hi:once_cell::sync::Lazy}} type, build on top of [`OnceCell`][c~once_cell::sync::OnceCell~docs]↗:

```rust,editable
{{#include ../../../crates/cats/memory_management/examples/lazy_initialization/once_cell2.rs:example}}
```

## `lazy_static` {#lazy-static}

[![lazy_static][c~lazy_static~docs~badge]][c~lazy_static~docs]{{hi:lazy_static}}
[![lazy_static~crates.io][c~lazy_static~crates.io~badge]][c~lazy_static~crates.io]
[![lazy_static~github][c~lazy_static~github~badge]][c~lazy_static~github]
[![lazy_static~lib.rs][c~lazy_static~lib.rs~badge]][c~lazy_static~lib.rs]
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}
[![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}}
[![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

[`lazy_static`][c~lazy_static~docs]↗{{hi:lazy_static}}

```rust,editable
{{#include ../../../crates/cats/memory_management/examples/lazy_initialization/lazy_static.rs:example}}
```

## Declare Lazily Evaluated Constants {#declare-lazily-evaluated-constant}

[![lazy_static][c~lazy_static~docs~badge]][c~lazy_static~docs]{{hi:lazy_static}}
[![lazy_static~crates.io][c~lazy_static~crates.io~badge]][c~lazy_static~crates.io]
[![lazy_static~github][c~lazy_static~github~badge]][c~lazy_static~github]
[![lazy_static~lib.rs][c~lazy_static~lib.rs~badge]][c~lazy_static~lib.rs]
[![cat~caching][cat~caching~badge]][cat~caching]{{hi:Caching}}
[![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}
[![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}}

Declares a lazily evaluated constant [`std::collections::HashMap`][c~std::collections::HashMap~docs]↗{{hi:std::collections::HashMap}}. The [`std::collections::HashMap`][c~std::collections::HashMap~docs]↗{{hi:std::collections::HashMap}} will be evaluated once and stored behind a global static reference.

```rust,editable
{{#include ../../../crates/cats/memory_management/examples/lazy_initialization/lazy_constant.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[lazy_initialization: write / fix NOW](https://github.com/john-cd/rust_howto/issues/411)
cover / link to Once OnceCell OnceLock Lazylock.

</div>
