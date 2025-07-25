# Smart Pointers

{{#include smart_pointers.incl.md}}

Smart pointers{{hi:Smart pointers}} are special data structures that act as pointers / references to data, while including additional functionality, such as:

- _Ownership and Borrowing_ allowing for shared ownership (`Rc`, `Arc`) or unique ownership (`Box`), while also providing borrowing capabilities.
- _Automatic memory management_, such as allocation and deallocation of heap memory.
- _Reference counting_, allowing multiple parts of a program to share ownership of data. Reference Counting is a mechanism where a count is kept of how many references point to a piece of data. When the count drops to zero, the data is deallocated.
- _Interior Mutability_ is the ability to mutate data even when you only have an immutable reference to it. This is typically achieved through smart pointers that enforce borrowing rules at runtime (e.g., `RefCell`, `Mutex`).
- _Thread Safety_ ensures that data is accessed and modified correctly when multiple threads are involved, preventing race conditions and other concurrency bugs.

Smart pointers are used to manage memory and data access in a way that is safe, efficient, and idiomatic in Rust.

Here's a table of common smart pointers in Rust, outlining their primary use cases and characteristics:

| Smart Pointer | Description | Use Cases | Example |
|---|---|---|---|
| `Box<T>` | A simple pointer to data allocated _on the heap_: Single owner of the data; allocates on heap; deallocates when `Box` goes out of scope. | Storing data on the heap when the size isn't known at compile time, or to avoid stack overflow for large data. Recursive data structures (e.g., `Cons` lists). | `let b = Box::new(5);` |
| `Rc<T>` | A reference-counting smart pointer: Allows multiple owners of the same data. Multiple immutable owners, data is dropped when the last `Rc` goes out of scope. Cannot be used for mutable data directly. | Best for scenarios where multiple parts of your program need access to the same data. Not thread-safe. Graph-like structures. | `let a = Rc::new(String::from("hello")); let b = Rc::clone(&a);` |
| `Arc<T>` | An atomically reference-counted smart pointer: Similar to `Rc<T>` but thread-safe, enabling shared ownership across threads: Multiple immutable owners, data is dropped when the last `Arc` goes out of scope. | Sharing data between multiple threads where each thread needs shared ownership. | `let data = Arc::new(vec![1, 2, 3]); let handle = thread::spawn(move \|\| { println!("{data:?}"); });` |
| `RefCell<T>` | Provides interior mutability (mutability inside an immutable reference) for data that is owned by `Rc<T>` (or other single-owner types). Enforces borrowing rules at runtime. Single owner of the `RefCell`, but allows mutable borrows to the inner data at runtime. Panics if borrowing rules are violated. | When you have an `Rc<T>` (or similar) and need to modify the data it points to, but only one mutable borrow is allowed at a time. | `let cell = Rc::new(RefCell::new(5)); *cell.borrow_mut() += 1;` |
| `Mutex<T>` | Provides mutual exclusion, allowing only one thread at a time to access the wrapped data. Thread-safe, provides interior mutability across threads. Blocks other threads trying to acquire the lock. | Sharing mutable data safely between multiple threads. | `let m = Arc::new(Mutex::new(5)); let data = m.lock().unwrap(); *data += 1;` |
| `RwLock<T>` | Provides a multiple-reader, single-writer lock. Allows multiple readers or one writer at a time. Thread-safe, provides interior mutability across threads. More permissive than `Mutex` for read operations. | When you have data that is frequently read but infrequently written to by multiple threads. | `let lock = Arc::new(RwLock::new(vec![1, 2, 3])); let read_guard = lock.read().unwrap();` |
| `Weak<T>` | A non-owning, "weak" reference to data managed by `Rc<T>` or `Arc<T>`. Does not prevent the data from being dropped. | Does not increase the reference count. Can be upgraded to `Rc<T>` or `Arc<T>` if the data still exists. Used to break reference cycles. | Preventing memory leaks in cyclic data structures when using `Rc<T>` or `Arc<T>`. Implementing caches or observers. | `let five = Rc::new(5); let weak_five = Rc::downgrade(&five);` |

## Smart Pointer Comparison {#comparison}

- `Rc<T>`{{hi:Rc<T>}} enables multiple owners{{hi:Multiple owners}} of the same data; `Box<T>` and `RefCell<T>` have single owners.
- `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows checked at compile time; `RefCell<T>` allows immutable or mutable borrows checked at runtime{{hi:Borrowing}}.

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
[add cow](https://github.com/john-cd/rust_howto/issues/1384)
add oncecell, std::sync::OnceLock, lazylock
</div>
