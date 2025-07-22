# Drop

{{#include drop.incl.md}}

## Release External Resources by Implementing the `Drop` Trait {#drop}

[![std][c~std~docs~badge]][c~std~docs]

The `Drop` trait allows you to define custom behavior when your types go out of scope, such as releasing resources or performing cleanup tasks.

Its `drop` method is called automatically by Rust when:

- A value goes out of scope (e.g., at the end of a function, a `let` binding's scope, or a block).
- A variable is re-assigned (and the old value is dropped).
- A collection (like a Vec) is dropped, its elements are also dropped.

Common use cases include:

- Releasing Memory or Resources: While Rust typically handles memory automatically, `Drop` is implemented by types that manage external resources, like file handles, network connections, mutex locks, or graphics contexts. When the `Drop` implementation runs, it can close the files, disconnect sockets, unlock mutexes, etc.
- Notifying external systems when an object is discarded.
- Logging.
- Implementing Smart Pointers: Types like `Box<T>`, `Rc<T>`, `String` and `Vec`, rely heavily on the `Drop` trait to manage their memory and resource deallocation. For instance, `Rc<T>` uses `Drop` to decrement its reference count and deallocate the data when the count reaches zero.

To implement `Drop` for a custom type, you would do something like this:

```rust,editable
{{#include ../../crates/cats/standard_library/examples/drop/drop.rs:example}}
```

## Related Topics {#skip}

- [[data_structures | Data Structures]].
- [[generics | Generics]].
- [[memory-management | Memory Management]].
- [[ownership_borrowing | Ownership & Borrowing]].
- [[reference_counting | Reference Counting]].
- [[smart_pointers | Smart Pointers]].
- [[traits | Traits]].
- [[vectors | Vectors]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
