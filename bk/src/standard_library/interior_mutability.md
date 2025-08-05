# Interior Mutability with `RefCell`, `Cell`, and `OnceCell`

The core idea of interior mutability is to allow data to be mutated even when there are immutable references to it. This is a deviation from Rust's usual compile-time borrowing rules (either _multiple_ immutable references or _one_ mutable reference) and instead enforces these rules at runtime. This pattern is useful for shared state within a single thread, such as in GUI applications or complex data structures.

`RefCell`, `Cell`, and `OnceCell`provide flexibility in managing mutable state in Rust, especially when the strict compile-time borrow checker is too restrictive for certain single-threaded patterns, by shifting the borrow checking to runtime.

`RefCell<T>` (and `Cell<T>`, `OnceCell<T>`) do not implement `Sync` and are therefore _single-threaded_. The corresponding `Sync` version of `RefCell<T>` is `RwLock<T>`. Use `Mutex<T>`, `RwLock<T>`, `OnceLock<T>`, or atomic types when working with multiple threads.

## `RefCell` {#refcell}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![book~rust~refcell][book~rust~refcell~badge]][book~rust~refcell]{{hi:RefCell}}

Rust memory safety allows (i) several immutable references (`&T`) to an object `T`; or (ii) _one_ mutable reference (`&mut T`). This is enforced _at compile time_. However, sometimes it is required to have multiple references to an object and yet mutate it. `RefCell<T>` (and related types `Cell<T>` and `OnceCell<T>`) have _interior mutability_, a pattern that allows you to mutate data even when there are immutable references to it.

These types are used in scenarios involving [shared state][p~shared-state] within a single thread, like GUI applications or when creating complex [data structures][p~data-structures] like graphs.

`RefCell` keeps track of borrowing rules _at runtime_ and ensures that only one mutable or multiple immutable borrows exist at a time.
Attempts to violate borrowing rules (like having multiple mutable borrows) will cause a _panic_ at runtime. Common methods include `borrow`, [`borrow_mut`][c~std::borrow::BorrowMut~docs]↗{{hi:borrow_mut}}, and [`try_borrow`][c~std::cell::RefCell::try_borrow~docs].

```rust,editable
{{#include ../../crates/standard_library/examples/interior_mutability/refcell.rs:example}}
```

## `Cell` {#cell}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

`Cell<T>` is a type that provides simple, byte-wise copy-able mutability. `Cell<T>` implements interior mutability by _moving values_ in and out of the cell. An `&mut T` to the inner value can never be obtained (unless `Cell` itself is mutable), and the value itself cannot be directly obtained without replacing it with something else.

`Cell<T>` is used when you need to mutate a value without using a reference or a mutable reference. Common methods include `set`, `get`, and `replace`. It is most often used for types that implement the `Copy` trait, like integers and booleans.

```rust,editable
{{#include ../../crates/standard_library/examples/interior_mutability/cell.rs:example}}
```

## `OnceCell` {#oncecell}

See [lazy initialization][p~lazy-initialization].

## Related Topics {#related-topics}

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

<div class="hidden">
[finish to rewrite OnceCell / example: RefCell inside of Rc / link to Mutex / RwLock](https://github.com/john-cd/rust_howto/issues/1385)
</div>
