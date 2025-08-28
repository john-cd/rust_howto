# Reference Counting

{{#include reference_counting.incl.md}}

## Share Ownership of a Value with `Rc` {#rc}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![book~rust~rc][book~rust~rc~badge]][book~rust~rc]{{hi:Rc}}{{hi:Multiple owners}}{{hi:Shared ownership}}

The [`Rc<T>`][c~std::rc::Rc~docs]↗{{hi:std::rc::Rc}} type (for "Reference Counted") enables _shared ownership_ of a value.

- `Rc` maintains a reference count of the number of owners. You can create additional references to the data using the [`clone`][c~std::clone::Clone~docs]↗{{hi:std::clone::Clone}} method. Cloning an `Rc` only increments the reference count without duplicating the data. When the last owner goes out of scope, the data is automatically cleaned up (dropped).
- If you need mutability, put a [`Cell`][c~std::cell::Cell~docs]↗{{hi:std::cell::Cell}} or [`RefCell`][c~std::cell::RefCell~docs]↗{{hi:std::cell::RefCell}} inside the `Rc`.
- `Rc` automatically dereferences to `T` (via the [`Deref`][c~std::ops::Deref~docs]↗{{hi:std::ops::Deref}} trait), so you can call `T`'s methods on a value of type `Rc<T>`.
- `Rc` is commonly used in [data structures][p~data-structures], such as graphs and linked lists, where multiple nodes might need to share ownership of certain nodes or data.
- Keep in mind that `Rc` is not thread-safe. For concurrent scenarios, you should use [`Arc`][c~std::sync::Arc~docs]↗{{hi:std::sync::Arc}}, which provides similar functionality with thread safety.

The following example demonstrates common operations with `Rc`:

```rust,editable
{{#include ../../crates/standard_library/examples/reference_counting/rc.rs:example}}
```

Also of note: the [`Weak`][c~std::rc::Weak~docs]↗{{hi:std::rc::Weak}} (weak reference) type, typically obtained via `Rc::downgrade`, allows for non-owning (weak) references to the data. This can help prevent reference cycles that could lead to memory leaks.

## Share Ownership of a Value with Interior Mutability via `Rc` and `RefCell` {#rc-refcell}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

[`Rc<T>`][c~std::rc::Rc~docs]↗{{hi:std::rc::Rc}} allows you to share data between multiple parts of your program but it doesn't allow unrestricted _mutability_ by itself. If `Rc<T>` allowed multiple mutable references, it may violate Rust's borrowing rules: multiple mutable borrows to the same place can cause data races and inconsistencies.

If you need mutability inside an `Rc`, you can use [`RefCell<T>`][c~std::cell::RefCell~docs]↗{{hi:std::cell::RefCell}}, which offers _interior mutability_: `RefCell<T>` enforces borrowing rules _at runtime_ instead of compile-time. It allows mutable borrows to the inner data at runtime, but _panics_ if borrowing rules are violated (again, only one mutable borrow is allowed at a time).

See [[interior_mutability | Interior Mutability]] for more details.

## Related Topics {#related-topics .skip}

- [[box | Box]].
- [[cow | Clone on Write]].
- [[data-structures | Data Structures]].
- [[interior_mutability | Interior Mutability]].
- [[lifetimes | Lifetimes]].
- [[ownership_borrowing | Ownership and Borrowing]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">

- [[concurrency | Concurrency]].

</div>
