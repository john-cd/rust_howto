# Release Resources when a Value is No Longer Needed

{{#include drop.incl.md}}

When a value is no longer needed, Rust will run a "destructor" on that value. The most common way that a value is no longer needed is when it goes out of scope. This destructor calls `Drop::drop` for that value, if the special `Drop` trait is implemented for its type.

## Release External Resources by Implementing the `Drop` Trait {#drop}

[![std][c~std~docs~badge]][c~std~docs]{{hi:Drop}}

The [`Drop`][c~std::ops::Drop~docs]↗{{hi:std::ops::Drop}} trait allows you to define custom behavior when your types go out of scope, such as releasing resources or performing cleanup tasks.

Its `drop` method is called automatically when:

- A value goes out of scope (e.g., at the end of a function, a [`let`][keyword~let]↗{{hi:let}} binding's scope, or a block).
- A variable is re-assigned (and the old value is dropped).
- A collection (like a `Vec`) is dropped - its elements are also dropped.

Common use cases include:

- Releasing memory or resources: While Rust typically handles memory automatically, `Drop` is implemented by types that manage external resources, like file handles, network connections, mutex locks, or graphics contexts. When the `Drop` implementation runs, it can close the files, disconnect sockets, unlock mutexes, etc.
- Notifying external systems when an object is discarded.
- Logging.
- Implementing [[smart_pointers | smart pointers]]: Types like [`Box<T>`][c~std::boxed::Box~docs]↗{{hi:std::boxed::Box}}, `Rc<T>`, `String` and `Vec<T>`, rely heavily on the `Drop` trait to manage their memory and resource deallocation. For instance, `Rc<T>` uses `Drop` to decrement its reference count and deallocate the data when the count reaches zero.

To implement `Drop` for a custom type, you would do something like this:

```rust,editable
{{#include ../../crates/standard_library/examples/drop/drop.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[data_structures | Data Structures]].
- [[generics | Generics]].
- [[ownership_borrowing | Ownership & Borrowing]].
- [[reference_counting | Reference Counting]].
- [[smart_pointers | Smart Pointers]].
- [[traits | Traits]].
- [[vectors | Vectors]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">

- [[memory-management | Memory Management]].

</div>
