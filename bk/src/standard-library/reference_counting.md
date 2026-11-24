<!-- TODO Reviewed ??-??-2026. -->

# Reference Counting

{{#include reference_counting.incl.md}}

[![Rust by Example - rc][book~rust-by-example~rc~badge]][book~rust-by-example~rc]

## Share Ownership of a Value with `Rc` {#rc}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![book~rust~rc][book~rust~rc~badge]][book~rust~rc]{{hi:Rc}}{{hi:Multiple owners}}{{hi:Shared ownership}}

The [`Rc<T>`][c~std::rc::Rc~docs]↗{{hi:std::rc::Rc}} type (for "Reference Counted") enables _shared ownership_ of a value.

- Calling [`clone`][c~std::clone::Clone~docs]↗{{hi:std::clone::Clone}} creates another owner of the same value and increments its strong reference count. Dropping the last strong owner drops the value.
- If you need mutability, put a [`Cell`][c~std::cell::Cell~docs]↗{{hi:std::cell::Cell}} or [`RefCell`][c~std::cell::RefCell~docs]↗{{hi:std::cell::RefCell}} inside the `Rc`.
- `Rc` automatically dereferences to `T` (via the [`Deref`][c~std::ops::Deref~docs]↗{{hi:std::ops::Deref}} trait), so you can call `T`'s methods on a value of type `Rc<T>`.
- `Rc` is commonly used in [data structures][p~data-structures], such as graphs and linked lists, where multiple nodes might need to share ownership of certain nodes or data.
- Use `Rc` within one thread. To share ownership across threads, use [`Arc`][c~std::sync::Arc~docs]↗{{hi:std::sync::Arc}} with a thread-safe inner type. `Arc` makes the reference count atomic.

The following example demonstrates common operations with `Rc`:

```rust,editable
{{#include ../../crates/standard_library/examples/reference_counting/rc.rs:example}}
```

Use `Rc::downgrade` to create a [`Weak`][c~std::rc::Weak~docs]↗{{hi:std::rc::Weak}} reference when a link should not keep its target alive. For example, a tree can use strong references to children and weak references back to parents.

## Share Ownership of a Value with Interior Mutability via `Rc` and `RefCell` {#rc-refcell}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

Cloning an `Rc<T>` gives each owner shared access to the same `T`.

Wrap the value in [`RefCell<T>`][c~std::cell::RefCell~docs]↗{{hi:std::cell::RefCell}} when several owners need to mutate it. `borrow()` permits multiple readers. `borrow_mut()` requires exclusive access until its guard is dropped. Both methods panic on a conflicting borrow. Use `try_borrow()` or `try_borrow_mut()` when the caller should handle that conflict.

See [[interior_mutability | Interior Mutability]] for more details.

For examples of sharing and coordinating mutation across threads, see [Shared-State Concurrency (draft)](../categories/concurrency/shared_state.md).

## Related Topics {#related-topics .skip}

- [[heap_storage | Box]].
- [[clone-on-write | Clone on Write]].
- [[data-structures | Data Structures]].
- [[interior_mutability | Interior Mutability]].
- [[lifetimes | Lifetimes]].
- [[ownership_and_borrowing | Ownership and Borrowing]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
