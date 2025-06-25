# Reference Counting

{{#include rc.incl.md}}

## Share Ownership of a Value with `Rc` {#rc}

[![std][c-std-badge]][c-std]{{hi:std}} [![book-rust-rc][book-rust-rc-badge]][book-rust-rc]{{hi:Rc}}{{hi:Multiple owners}}{{hi:Shared ownership}}

The `Rc<T>` type (for "Reference Counted") enables _shared ownership_ of a value.

- `Rc` maintains a reference count of the number of owners. You can create additional references to the data using the [`clone`][c-std::clone::Clone]⮳{{hi:clone}} method. Cloning an `Rc` only increments the reference count without duplicating the data. When the last owner goes out of scope, the data is automatically cleaned up (dropped).
- If you need mutability, put a `Cell` or `RefCell` inside the `Rc`.
- `Rc` automatically dereferences to `T` (via the [`Deref`][c-std::ops::Deref]⮳{{hi:Deref}} trait), so you can call `T`'s methods on a value of type `Rc<T>`.
- `Rc` is commonly used in [data structures][p-data-structures], such as graphs and linked lists, where multiple nodes might need to share ownership of certain nodes or data.
- Keep in mind that `Rc` is not thread-safe. For concurrent scenarios, you should use `Arc` (Atomic Reference Counted), which provides similar functionality with thread safety.

The following example demonstrates common operations with `Rc`:

```rust,editable
{{#include ../../crates/standard_library/examples/smart_pointers/rc.rs:example}}
```

Also of note: the `Weak` type, typically obtained via `Rc::downgrade`, allows for non-owning (weak) references to the data. This can help prevent reference cycles that could lead to memory leaks.

## Interior Mutability with `Rc` and `RefCell` {#rc-refcell}

[![std][c-std-badge]][c-std]{{hi:std}}

`Rc<T>` allows you to share data between multiple parts of your program but it doesn't allow _mutability_ by itself. If `Rc<T>` allowed multiple mutable references, it may violate Rust's borrowing rules: multiple mutable borrows to the same place can cause data races and inconsistencies.

If you need mutability inside an `Rc`, you can use `RefCell<T>`, which offers _interior mutability_: `RefCell<T>` enforces borrowing rules at runtime instead of compile-time. It allows mutable borrows to the inner data at runtime, but panics if borrowing rules are violated (again, only one mutable borrow is allowed at a time).

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
