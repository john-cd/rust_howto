# Interior Mutability with `RefCell`, `Cell`, and `OnceCell`

{{#include interior_mutability.incl.md}}

The core idea of interior mutability is to allow data to be mutated even when there are immutable references to it. This is a deviation from Rust's usual compile-time borrowing rules (either _multiple_ immutable references or _one_ mutable reference) and instead enforces these rules at runtime. This pattern is useful for shared state within a single thread, such as in GUI applications or complex data structures.

[`RefCell`][c~std::cell::RefCell~docs]↗{{hi:std::cell::RefCell}}, [`Cell`][c~std::cell::Cell~docs]↗{{hi:std::cell::Cell}}, and [`OnceCell`][c~std::cell::OnceCell~docs]↗{{hi:std::cell::OnceCell}} provide flexibility in managing mutable state in Rust, especially when the strict compile-time borrow checker is too restrictive for certain single-threaded patterns, by shifting the borrow checking to runtime.

`RefCell<T>` (and `Cell<T>`, `OnceCell<T>`) do not implement `Sync` and are therefore _single-threaded_. The corresponding `Sync` version of `RefCell<T>` is [`RwLock<T>`][c~std::sync::RwLock~docs]↗{{hi:std::sync::RwLock}}. Use [`Mutex<T>`][c~std::sync::Mutex~docs]↗{{hi:std::sync::Mutex}}, `RwLock<T>`, [`OnceLock<T>`][c~std::sync::OnceLock~docs]↗{{hi:std::sync::OnceLock}}, or atomic types when working with multiple threads.

## Use `RefCell` {#refcell}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![book~rust~refcell][book~rust~refcell~badge]][book~rust~refcell]{{hi:std::cell::RefCell}}

Rust memory safety allows (i) several immutable references (`&T`) to an object `T`; or (ii) _one_ mutable reference (`&mut T`). This is enforced _at compile time_. However, sometimes it is required to have multiple references to an object and yet mutate it. `RefCell<T>` (and related types `Cell<T>` and `OnceCell<T>`) have _interior mutability_, a pattern that allows you to mutate data even when there are immutable references to it.

These types are used in scenarios involving [[shared-state | shared state]] within a single thread, like GUI applications or when creating complex [data structures][p~data-structures] like graphs.

`RefCell` keeps track of borrowing rules _at runtime_ and ensures that only one mutable or multiple immutable borrows exist at a time.
Attempts to violate borrowing rules (like having multiple mutable borrows) will cause a _panic_ at runtime. Common methods include `borrow`, [`borrow_mut`][c~std::borrow::BorrowMut~docs]↗, and [`try_borrow`][c~std::cell::RefCell::try_borrow~docs]↗.

```rust,editable
{{#include ../../crates/standard_library/examples/interior_mutability/refcell.rs:example}}
```

### Shared Mutable State with `Rc<RefCell<T>>` {#rc-refcell}

A common pattern in Rust is using [`Rc<T>`][c~std::rc::Rc~docs]↗{{hi:std::rc::Rc}} together with `RefCell<T>` to allow multiple owners of mutable data. `Rc<T>` allows multiple owners of some data, but it only gives immutable access to that data. By wrapping a `RefCell<T>` inside an `Rc<T>`, you get a value that can have multiple owners _and_ can be mutated!

```rust,editable
{{#include ../../crates/standard_library/examples/interior_mutability/rc_refcell.rs:example}}
```

If you need shared mutable state across multiple threads, you should use `Arc<T>` with [`Mutex<T>`][c~std::sync::Mutex~docs]↗{{hi:std::sync::Mutex}} or [`RwLock<T>`][c~std::sync::RwLock~docs]↗{{hi:std::sync::RwLock}} instead of `Rc<RefCell<T>>`.

## Use `Cell` {#cell}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}{{hi:std::cell::Cell}}

[`Cell<T>`][c~std::cell::Cell~docs]↗ is a type that provides simple, byte-wise copy-able mutability. `Cell<T>` implements interior mutability by _moving values_ in and out of the cell. An `&mut T` to the inner value can never be obtained (unless `Cell` itself is mutable), and the value itself cannot be directly obtained without replacing it with something else.

`Cell<T>` is used when you need to mutate a value without using a reference or a mutable reference. Common methods include `set`, `get`, and `replace`. It is most often used for types that implement the [`Copy`][c~std::marker::Copy~docs]↗{{hi:std::marker::Copy}} trait, like integers and booleans.

```rust,editable
{{#include ../../crates/standard_library/examples/interior_mutability/cell.rs:example}}
```

## Use `OnceCell` {#oncecell}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}{{hi:std::cell::OnceCell}}

[`OnceCell<T>`][c~std::cell::OnceCell~docs]↗ allows for single assignment interior mutability. Unlike `Cell` or `RefCell`, once a `OnceCell` is initialized, its value cannot be changed. This makes it perfect for [[lazy-initialization | lazy initialization]], where you want to defer the creation of an expensive value until it is first needed, but ensure it is only computed once.

You can initialize a `OnceCell` using `set`, but a more common pattern is to use `get_or_init`, which takes a closure that returns the initialization value.

```rust,editable
{{#include ../../crates/standard_library/examples/interior_mutability/once_cell.rs:example}}
```

For a multi-threaded context, use [`OnceLock<T>`][c~std::sync::OnceLock~docs]↗{{hi:std::sync::OnceLock}} instead of `OnceCell<T>`.

## Related Topics {#related-topics .skip}

- [[concurrency | Concurrency]].
- [[data-structures | Data Structures]].
- [[memory-management | Memory Management]].
- [[memory_usage_analysis | Memory Usage Analysis]].
- [[reference_counting | Reference Counting]].
- [[rust-patterns | Rust Patterns]].
- [[shared_state | Shared State]].
- [[smart_pointers | Smart Pointers]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
