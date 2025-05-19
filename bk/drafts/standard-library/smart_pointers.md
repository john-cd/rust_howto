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
