<!-- TODO Reviewed ??-??-2026 -->

# Global Statics and Lazy Initialization

{{#include lazy_initialization.incl.md}}

Rust distinguishes compile-time initialized `static` values from values created at runtime. For runtime initialization, choose a cell based on whether callers need to supply a value or a closure, and whether access must be thread-safe:

- `OnceCell` / `OnceLock` accept a value or initializer the first time it is set.
- `LazyCell` / `LazyLock` hold an initializer and run it on first access.
- `std::cell` types are for single-threaded use. `std::sync` types can be shared across threads.

`OnceCell<T>` and `OnceLock<T>` have been stable since Rust 1.70. `LazyCell<T>` and `LazyLock<T>` have been stable since Rust 1.80. Prefer the standard library when the project's minimum Rust version supports the type. Use [`once_cell`][c~once_cell~docs]↗ when supporting an older compiler or when its additional APIs fit the project. [`lazy_static`][c~lazy_static~docs]↗ remains a supported macro-based alternative, especially in older code.

Lazy initialization is not a replacement for synchronization of later mutations. Use a [`Mutex`][c~std::sync::Mutex~docs]↗ or [`RwLock`][c~std::sync::RwLock~docs]↗ around mutable shared state, and consider whether a global is needed at all.

## Lazy Initialize Variables {#std}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}
[![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}}

Use the single-threaded `std::cell` types when access never crosses threads. They do not provide synchronization.

- The [`OnceCell`][c~core::cell::OnceCell~docs]↗{{hi:OnceCell}} type provides a way to define a value that will be initialized at most once.
- The corresponding thread-safe version of `OnceCell<T>` is [`OnceLock<T>`][c~std::sync::OnceLock~docs]↗. It allows a value to be initialized once, even when multiple threads race to access it.
- [`LazyCell<T>`][c~std::cell::LazyCell~docs]↗ is a value which is initialized on the first access. It is not thread-safe.
- [`LazyLock`][c~std::sync::LazyLock~docs]↗ is the thread-safe counterpart of `LazyCell` and can be used in statics. Access waits while another thread runs the initializer.
- [`Once`][c~std::sync::Once~docs]↗ is a low-level primitive that allows initialization of a value exactly once, ensuring that the initialization code is executed only once, even in the presence of multiple threads.

```rust,editable
{{#include ../../../crates/cats/memory_management/examples/lazy_initialization/once_cell.rs:example}}
```

For shared values, use `OnceLock`; it can be initialized on demand from any thread:

```rust,editable
{{#include ../../../crates/cats/memory_management/examples/lazy_initialization/once_lock.rs:example}}
```

Use `LazyCell` when the initializer should be stored with the value and access is single-threaded:

```rust,editable
{{#include ../../../crates/cats/memory_management/examples/lazy_initialization/lazy_cell.rs:example}}
```

For shared global data with a fixed initializer, use `LazyLock`:

```rust,editable
{{#include ../../../crates/cats/memory_management/examples/lazy_initialization/lazy_lock.rs:example}}
```

## `once_cell` {#once-cell}

{{#badges once_cell memory-management="Memory management" rust-patterns="Rust patterns"}}

The [`once_cell`][c~once_cell~docs]↗ crate provides `unsync::OnceCell` and `sync::OnceCell`, along with lazy wrappers. Its `sync::Lazy` is useful when supporting an older Rust version or when depending on the crate already.

```rust,editable
{{#include ../../../crates/cats/memory_management/examples/lazy_initialization/once_cell2.rs:example}}
```

The standard library types cover common new-code use cases. Avoid adding both `once_cell` and `lazy_static` to a new project unless their APIs or compiler support are needed.

## `lazy_static` {#lazy-static}

{{#badges lazy_static no-std="No standard library" memory-management="Memory management" rust-patterns="Rust patterns"}}

`lazy_static!` is a supported legacy alternative. Since Rust 1.80, prefer the standard library's `LazyLock` for the same thread-safe lazy initialization pattern in new code.

```rust,editable
{{#include ../../../crates/cats/memory_management/examples/lazy_initialization/lazy_static.rs:example}}
```

## Declare Lazily Evaluated Constants {#declare-lazily-evaluated-constant}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}
[![cat~caching][cat~caching~badge]][cat~caching]{{hi:Caching}}
[![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}
[![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}}

This example uses `std::sync::LazyLock` to define a global map initialized on first access. The map is immutable after initialization, so it does not need a mutex.

```rust,editable
{{#include ../../../crates/cats/memory_management/examples/lazy_initialization/lazy_constant.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[constants_and_statics | Constants and Statics]].
- [[interior_mutability | Interior Mutability]].
- [[smart_pointers | Smart Pointers]].
- [[concurrency | Concurrency]].
- [`std::sync::Once`][c~std::sync::Once~docs]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
