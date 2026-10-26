<!-- TODO Reviewed ??-??-2026 -->

# Resource Cleanup

{{#include resource_cleanup.incl.md}}

[![Rust by Example - raii][book~rust-by-example~raii~badge]][book~rust-by-example~raii]

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
- Implementing [[smart_pointers | smart pointers]]: [`Box<T>`][c~std::boxed::Box~docs]↗{{hi:std::boxed::Box}}, `String`, `Vec<T>`, and `Rc<T>` release owned resources when dropped. `Vec<T>` drops its elements and releases its buffer; `String` owns its bytes in a vector, whose field is also dropped. A type does not need its own `Drop` implementation just to have its fields cleaned up: compiler-generated drop glue drops fields recursively. For `Rc<T>`, the inner value is dropped when the last strong reference goes away; its allocation is freed after the weak references are gone as well.

Rust drops local variables when they leave scope, in reverse declaration order. Struct fields are dropped in declaration order after the type's `Drop::drop` method returns. Calling [`std::mem::drop`][c~std::mem::drop~docs]↗ explicitly drops a value early; `std::mem::forget` deliberately skips its destructor and can leak resources.

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

accuracy review: standard library

</div>
