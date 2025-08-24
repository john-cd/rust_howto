# Smart Pointers

{{#include smart_pointers.incl.md}}

Smart pointers{{hi:Smart pointers}} are special data structures that act as pointers / references to data, while including additional functionality, such as:

- _Ownership and Borrowing_ allowing for shared ownership ([`Rc`][c~std::rc::Rc~docs]↗{{hi:std::rc::Rc}}, [`Arc`][c~std::sync::Arc~docs]↗{{hi:std::sync::Arc}}) or unique ownership ([`Box`][c~std::boxed::Box~docs]↗{{hi:std::boxed::Box}}), while also providing borrowing capabilities.
- _Automatic memory management_, such as allocation and deallocation of heap memory.
- _Reference counting_, allowing multiple parts of a program to share ownership of data. Reference Counting is a mechanism where a count is kept of how many references point to a piece of data. When the count drops to zero, the data is deallocated.
- _Interior Mutability_ is the ability to mutate data even when you only have an immutable reference to it. This is typically achieved through smart pointers that enforce borrowing rules at runtime (e.g., [`RefCell`][c~std::cell::RefCell~docs]↗{{hi:std::cell::RefCell}}, [`Mutex`][c~std::sync::Mutex~docs]↗{{hi:std::sync::Mutex}}).
- _Thread Safety_ ensures that data is accessed and modified correctly when multiple threads are involved, preventing race conditions and other concurrency bugs.

Smart pointers are used to manage memory and data access in a way that is safe, efficient, and idiomatic in Rust.

Here's a table of common smart pointers in Rust, outlining their primary use cases and characteristics:

| Smart Pointer | Description | Use Cases | Example |
|---|---|---|---|
| [`Box<T>`][c~std::boxed::Box~docs]↗{{hi:std::boxed::Box}} | A simple pointer to data allocated _on the heap_: Single owner of the data; allocates on heap; deallocates when `Box` goes out of scope. | Storing data on the heap when the size isn't known at compile time, or to avoid stack overflow for large data. Recursive data structures (e.g., `Cons` lists). | `let b = Box::new(5);` |
| [`Rc<T>`][c~std::rc::Rc~docs]↗{{hi:std::rc::Rc}} | A reference-counting smart pointer: Allows multiple owners of the same data. Multiple immutable owners, data is dropped when the last `Rc` goes out of scope. Cannot be used for mutable data directly. | Best for scenarios where multiple parts of your program need access to the same data. Not thread-safe. Graph-like structures. | `let a = Rc::new(String::from("hello")); let b = Rc::clone(&a);` |
| [`Arc<T>`][c~std::sync::Arc~docs]↗{{hi:std::sync::Arc}} | An atomically reference-counted smart pointer: Similar to `Rc<T>` but thread-safe, enabling shared ownership across threads: Multiple immutable owners, data is dropped when the last `Arc` goes out of scope. | Sharing data between multiple threads where each thread needs shared ownership. | `let data = Arc::new(vec![1, 2, 3]); let handle = thread::spawn(move \|\| { println!("{data:?}"); });` |
| [`RefCell<T>`][c~std::cell::RefCell~docs]↗{{hi:std::cell::RefCell}} | Provides interior mutability (mutability inside an immutable reference) for data that is owned by `Rc<T>` (or other single-owner types). Enforces borrowing rules at runtime. Single owner of the `RefCell`, but allows mutable borrows to the inner data at runtime. Panics if borrowing rules are violated. | When you have an `Rc<T>` (or similar) and need to modify the data it points to, but only one mutable borrow is allowed at a time. | `let cell = Rc::new(RefCell::new(5)); *cell.borrow_mut() += 1;` |
| [`Mutex<T>`][c~std::sync::Mutex~docs]↗{{hi:std::sync::Mutex}} | Provides mutual exclusion, allowing only one thread at a time to access the wrapped data. Thread-safe, provides interior mutability across threads. Blocks other threads trying to acquire the lock. | Sharing mutable data safely between multiple threads. | `let m = Arc::new(Mutex::new(5)); let data = m.lock().unwrap(); *data += 1;` |
| [`RwLock<T>`][c~std::sync::RwLock~docs]↗{{hi:std::sync::RwLock}} | Provides a multiple-reader, single-writer lock. Allows multiple readers or one writer at a time. Thread-safe, provides interior mutability across threads. More permissive than `Mutex` for read operations. | When you have data that is frequently read but infrequently written to by multiple threads. | `let lock = Arc::new(RwLock::new(vec![1, 2, 3])); let read_guard = lock.read().unwrap();` |
| [`Weak<T>`][c~std::rc::Weak~docs]↗{{hi:std::rc::Weak}}{{hi:std::sync::Weak}} | A non-owning, "weak" reference to data managed by `Rc<T>` or `Arc<T>`. Does not prevent the data from being dropped. | Does not increase the reference count. Can be upgraded to `Rc<T>` or `Arc<T>` if the data still exists. Used to break reference cycles. | Preventing memory leaks in cyclic data structures when using `Rc<T>` or `Arc<T>`. Implementing caches or observers. | `let five = Rc::new(5); let weak_five = Rc::downgrade(&five);` |

## Smart Pointer Comparison {#smart-pointer-comparison}

- `Rc<T>`{{hi:std::rc::Rc}} and `Arc<T>`{{hi:std::sync::Arc}} are enables multiple owners{{hi:Multiple owners}} of the same data; [`Box<T>`][c~std::boxed::Box~docs]↗{{hi:std::boxed::Box}} and `RefCell<T>` have single owners.
- `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows checked at compile time; `RefCell<T>` allows immutable or mutable borrows checked at runtime{{hi:Borrowing}}.

## Multiple Ownership with Reference Counting {#reference-counting}

[![std][c~std~docs~badge]][c~std~docs]

[`Rc`][c~std::rc::Rc~docs]↗{{hi:std::rc::Rc}} and [`Arc`][c~std::sync::Arc~docs]↗{{hi:std::sync::Arc}} allow multiple owners of the same data. See [[reference_counting | Reference Counting]].

## Interior Mutability {#interior-mutability}

[![std][c~std~docs~badge]][c~std~docs]

[`RefCell<T>`][c~std::cell::RefCell~docs]↗{{hi:std::cell::RefCell}} provides interior mutability (mutability inside an immutable reference) for data that is owned by `Rc<T>` (or other single-owner types). Enforces borrowing rules at runtime.

See [[interior_mutability | Interior Mutability]].

## Lazy Initialization {#lazy-initialization}

[![std][c~std~docs~badge]][c~std~docs]

The standard library provides several types for lazy initialization: [`LazyCell<T>`][c~std::cell::LazyCell~docs]↗, [`LazyLock`][c~std::sync::LazyLock~docs]↗, [`OnceLock<T>`][c~std::sync::OnceLock~docs]↗.

See details in the [[lazy_initialization | Lazy Initialization]] chapter.

## Synchronization Primitives {#sync-primitives}

[![std][c~std~docs~badge]][c~std~docs]

The [`std::sync`][c~std::sync~docs]↗{{hi:std::sync}} module provides synchronization primitives for concurrent programming. These primitives allow safe sharing of data between threads, ensuring that data is accessed in a thread-safe manner. The most commonly used synchronization primitives include:

- [`Arc`][c~std::sync::Arc~docs]↗{{hi:std::sync::Arc}} is a thread-safe, reference-counted pointer that allows multiple threads to share ownership of a value. It is similar to [`Rc`][c~std::rc::Rc~docs]↗{{hi:std::rc::Rc}}, but designed for concurrent use.
- [`Mutex`][c~std::sync::Mutex~docs]↗{{hi:std::sync::Mutex}} is a mutual exclusion primitive that provides safe access to data by allowing only one thread to access the data at a time. It ensures that data is not accessed concurrently.
- [`RwLock`][c~std::sync::RwLock~docs]↗{{hi:std::sync::RwLock}} is a read-write lock that allows multiple readers or a single writer to access the data. It is useful when reads are more frequent than writes, as it allows concurrent reads while ensuring exclusive access for writes.

Less common primitives include:

- [`Barrier`][c~std::sync::Barrier~docs]↗{{hi:std::sync::Barrier}} is a synchronization primitive that allows multiple threads to wait for each other at a certain point in their execution. It is useful for coordinating the execution of multiple threads that need to reach a certain point before proceeding.
- [`Condvar`][c~std::sync::Condvar~docs]↗{{hi:std::sync::Condvar}} is a condition variable that allows threads to wait for a condition to be met before proceeding. It is often used in conjunction with [`Mutex`][c~std::sync::Mutex~docs]↗{{hi:std::sync::Mutex}} to signal when a condition changes, allowing threads to wake up and continue execution.

See the [[concurrency | Concurrency]] chapter.

## Implement a Smart Pointer with `Deref` {#deref}

[![std][c~std~docs~badge]][c~std~docs]

The [`Deref`][c~std::ops::Deref~docs]↗{{hi:std::ops::Deref}} trait enables types to _behave like references_, providing access to the data they wrap.

`Deref` enables the `*` operator and _implicit_, _automatic dereferencing_ in many circumstances, especially in method calls (`.` operator), meaning users can call methods on the inner type as if they were working directly with it. This mechanism is called "Deref coercion".

Method resolution with [`Deref`][c~std::ops::Deref~docs]↗{{hi:std::ops::Deref}} kicks in when a type doesn't have a method directly defined on it, but it implements the `Deref` trait. The compiler will follow the `Deref` chain to find the method on the inner type. For example, if you have a `Box<String>`, and you call `.len()` on it, the compiler dereferences first from `Box` to `String`, then to `str`, and ultimately finds and calls `str::len`.

Types that implement [`Deref`][c~std::ops::Deref~docs]↗{{hi:std::ops::Deref}} or [`DerefMut`][c~std::ops::DerefMut~docs]↗{{hi:std::ops::DerefMut}} are called "smart pointers". Often, the purpose of such a type is to change the ownership semantics of a contained value (for example, [`Rc`][c~std::rc::Rc~docs]↗{{hi:std::rc::Rc}} or [`Cow`][c~std::borrow::Cow~docs]↗{{hi:std::borrow::Cow}}) or the storage semantics of a contained value (for example, [`Box`][c~std::boxed::Box~docs]↗{{hi:std::boxed::Box}}).

BEWARE: The compiler will silently insert calls to `Deref::deref`. For this reason, one should be careful about implementing [`Deref`][c~std::ops::Deref~docs]↗{{hi:std::ops::Deref}} and only do so when deref coercion is desirable.

In mutable contexts, [`DerefMut`][c~std::ops::DerefMut~docs]↗{{hi:std::ops::DerefMut}} is used and mutable deref coercion similarly occurs.

The [`AsRef`][c~std::convert::AsRef~docs]↗{{hi:std::convert::AsRef}} and `Borrow` traits have very similar signatures to `Deref`. It may be desirable to implement either or both of these, whether in addition to or rather than `Deref` traits. See [[asref | AsRef]] and [[borrow | Borrow]].

Read the [Treating Smart Pointers Like Regular References with Deref][book~rust~ch15-02-deref]↗ and the [dereference operator][book~rust-reference~dereference-operator]↗ for more details.

The following example demonstrates the implementation of a basic smart pointer:

```rust,editable
{{#include ../../crates/standard_library/examples/smart_pointers/deref.rs:example}}
```

## Related Topics {#related-topics}

- [[asref | AsRef]].
- [[borrow | Borrow]].
- [[cow | COW]] (Clone-on-Write).

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[add cow](https://github.com/john-cd/rust_howto/issues/1384)
add oncecell, std::sync::OnceLock, lazylock.

- [[concurrency | Concurrency]].
- [[memory-management | Memory Management]].
- [[memory_usage_analysis | Memory Usage Analysis]].
- [[rust-patterns | Rust Patterns]].
- [[shared_state | Shared State]].

</div>
