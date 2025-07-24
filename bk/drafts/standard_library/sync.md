# Synchronization Primitives

{{#include sync.incl.md}}

## Synchronization Primitives {#sync-primitives}

[![std][c~std~docs~badge]][c~std~docs]

The [`std::sync`](https://doc.rust-lang.org/std/sync/index.html)⮳{{hi:sync}} module provides synchronization primitives for concurrent programming. These primitives allow safe sharing of data between threads, ensuring that data is accessed in a thread-safe manner. The most commonly used synchronization primitives include:

- [`Arc<T>`](https://doc.rust-lang.org/std/sync/struct.Arc.html)⮳ is a thread-safe, reference-counted pointer that allows multiple threads to share ownership of a value. It is similar to `Rc<T>`, but designed for concurrent use.
- [`Mutex<T>`](https://doc.rust-lang.org/std/sync/struct.Mutex.html)⮳ is a mutual exclusion primitive that provides safe access to data by allowing only one thread to access the data at a time. It ensures that data is not accessed concurrently.
- `RwLock<T>` is a read-write lock that allows multiple readers or a single writer to access the data. It is useful when reads are more frequent than writes, as it allows concurrent reads while ensuring exclusive access for writes.
- [`LazyLock`](https://doc.rust-lang.org/std/sync/struct.LazyLock.html)⮳ is a lazily initialized value that is initialized on first access, ensuring that the initialization code is executed only once, even in the presence of multiple threads.

Less common primitives include:

- **`OnceLock<T>`** is a lock that allows a value to be initialized exactly once, ensuring that the initialization code is executed only once, even in the presence of multiple threads.
- **`Condvar`** is a condition variable that allows threads to wait for a condition to be met before proceeding. It is often used in conjunction with `Mutex<T>` to signal when a condition changes, allowing threads to wake up and continue execution.
- **`Once`** is a low-level primitive that allows initialization of a value exactly once, ensuring that the initialization code is executed only once, even in the presence of multiple threads.
- **`Barrier`** is a synchronization primitive that allows multiple threads to wait for each other at a certain point in their execution. It is useful for coordinating the execution of multiple threads that need to reach a certain point before proceeding.
- **`Semaphore`** is a counting semaphore that allows a limited number of threads to access a resource concurrently. It is useful for limiting the number of concurrent accesses to a resource, such as a database connection pool.

## Related Topics {#skip}

- [[concurrency | Concurrency]].
- [[smart_pointers | Smart Pointers]].

## References {#skip}

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
