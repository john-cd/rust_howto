# Smart Pointers

{{#include smart_pointers.incl.md}}

Smart pointers{{hi:Smart pointers}} are special data structures that not only act as pointers (to manage references to data) but also include additional functionality, such as automatic memory management.

| Smart Pointer | Description |
|---|---|
| `Box<T>` | A smart pointer for allocating values _on the heap_. Useful when you have a large amount of data or a value whose size is not known at compile time. |
| `Rc<T>` | A reference-counted smart pointer used for sharing ownership of data. Best for scenarios where multiple parts of your program need access to the same data. Not thread-safe. |
| `Arc<T>` | Uses atomic operations to ensure safe concurrent access. Similar to `Rc<T>` but thread-safe, enabling shared ownership across threads. |
| `RefCell<T>` | Allows interior mutability (inside an immutable reference). Enforces borrow checking at runtime rather than compile time. Works well with  when shared data requires interior mutability. |
| `Cell<T>` | Similar to , but with fewer safety checks and restrictions. Enables interior mutability for  types without borrowing. |

## Comparison {#skip}

- `Rc<T>`{{hi:Rc<T>}} enables multiple owners{{hi:Multiple owners}} of the same data; `Box<T>` and `RefCell<T>` have single owners.
- `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows checked at compile time; `RefCell<T>` allows immutable or mutable borrows checked at runtime{{hi:Borrowing}}.
- Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.

## `Rc` {#rc}

[![std][c-std-badge]][c-std]{{hi:std}} [![book-rust-rc][book-rust-rc-badge]][book-rust-rc]{{hi:Rc}}

The `Rc<T>` type (for "Reference Counted") enables shared ownership of a value.{{hi:Multiple owners}}{{hi:Shared ownership}}

- `Rc` maintains a reference count of the number of owners. You can create additional references to the data using the [`clone`][c-std::clone::Clone]⮳{{hi:clone}} method. Cloning an `Rc` only increments the reference count without duplicating the data. When the last owner goes out of scope, the data is automatically cleaned up (dropped).
- If you need mutability, put a `Cell` or `RefCell` inside the `Rc`.
- `Rc` automatically dereferences to `T` (via the [`Deref`][c-std::ops::Deref]⮳{{hi:Deref}} trait), so you can call `T`'s methods on a value of type `Rc<T>`.
- `Rc` is commonly used in [data structures][p-data-structures], such as graphs and linked lists, where multiple nodes might need to share ownership of certain nodes or data.

Keep in mind that `Rc` is not thread-safe. For concurrent scenarios, you should use `Arc` (Atomic Reference Counted), which provides similar functionality with thread safety.

```rust,editable
{{#include ../../crates/standard_library/tests/smart_pointers/rc.rs:example}}
```

Also of note: the `Weak` type, typically obtained via `Rc::downgrade`, allows for non-owning (weak) references to the data. This can help prevent reference cycles that could lead to memory leaks.

## `RefCell` {#refcell}

[![std][c-std-badge]][c-std]{{hi:std}} [![book-rust-refcell][book-rust-refcell-badge]][book-rust-refcell]{{hi:RefCell}}

Rust memory safety allows (i) several immutable references (`&T`) to an object `T`; or (ii) _one_ mutable reference (`&mut T`). This is enforced _at compile time_. However, sometimes it is required to have multiple references to an object and yet mutate it. `RefCell<T>` (and related types `Cell<T>` and `OnceCell<T>`) have _interior mutability_, a pattern that allows you to mutate data even when there are immutable references to it.

These types are used in scenarios involving [shared state][p-shared-state] within a single thread, like GUI applications, or when creating complex [data structures][p-data-structures] like graphs.

- `RefCell` keeps track of borrowing rules _at runtime_ and ensures that only one mutable or multiple immutable borrows exist at a time.
Attempts to violate borrowing rules (like having multiple mutable borrows) will cause a _panic_ at runtime. Common methods include `borrow`, [`borrow_mut`][c-std::borrow::BorrowMut]⮳{{hi:borrow_mut}}, and [`try_borrow`][c-std::cell::RefCell::try_borrow].

```rust,editable
{{#include ../../crates/standard_library/tests/smart_pointers/refcell.rs:example}}
```

- `RefCell<T>` (and `Cell<T>`, `OnceCell<T>`) do not implement `Sync` and are therefore _single-threaded_. The corresponding `Sync` version of `RefCell<T>` is `RwLock<T>`. Use `Mutex<T>`, `RwLock<T>`, `OnceLock<T>`, or atomic types when working with multiple threads.

## `Cell` {#cell}

`Cell<T>` is a type that provides simple, byte-wise copy-able mutability. It is commonly used for types that implement the `Copy` trait, like integers and booleans. `Cell<T>` is used when you need to mutate a value without using a reference or a mutable reference. Common methods include `set`, `get`, and [`replace`][c-regex::Regex::replace_all]⮳{{hi:replace}} [`replace`][c-regex::Replacer]⮳{{hi:replace}} .

```rust,editable
{{#include ../../crates/standard_library/tests/smart_pointers/cell.rs:example}}
```

"`Cell<T>` implements interior mutability by moving values in and out of the cell. That is, an `&mut T` to the inner value can never be obtained, and the value itself cannot be directly obtained without replacing it with something else."

## `OnceCell` {#oncecell}

- The `OnceCell` type provides a way to define a value that will be initialized at most once. It's useful for scenarios where you want [lazy initialization][p-lazy-initialization] without the overhead of thread synchronization.

## Related Data Structures {#skip}

- [[cow | COW]] (Copy-on-Write).

## Related Topics {#skip}

- [[concurrency | Concurrency]].
- [[memory-management | Memory Management]].
- [[memory_usage_analysis | Memory Usage Analysis]].
- [[rust-patterns | Rust Patterns]].
- [[shared_state | Shared State]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[smart_pointers: review NOW](https://github.com/john-cd/rust_howto/issues/628)
- finish to rewrite Cell, OnceCell.
- example: RefCell inside of Rc.
Mutex
</div>
