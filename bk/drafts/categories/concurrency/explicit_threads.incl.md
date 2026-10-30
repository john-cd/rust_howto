<!-- TODO Reviewed ??-??-2026 -->

`std::thread::spawn` starts an owned thread and returns a `JoinHandle<T>`. Keep the handle and call `join` to collect its result and detect a panic; dropping it detaches the thread. The process can exit while detached threads are still running.

Use `std::thread::scope` for short-lived workers that borrow stack data. It has been stable since Rust 1.63 and joins every scoped thread before the scope returns; explicitly join a handle when you need its value or want to report a panic.

Use `std::thread::Builder` when a worker needs a name or a custom stack size. Its `spawn` method is fallible. `std::thread::available_parallelism()` provides a hint for sizing a worker pool, subject to operating-system limits.

| Recipe                                                                   | Crates         | Categories                              |
| ------------------------------------------------------------------------ | -------------- | --------------------------------------- |
| [Spawn Threads and Collect Their Results][ex~concurrency~spawn-join]     | {{#crate std}} | {{#categories concurrency=Concurrency}} |
| [Use Scoped Threads to Borrow Stack Data][ex~concurrency~scoped-threads] | {{#crate std}} | {{#categories concurrency=Concurrency}} |
| [Name Threads and Set the Stack Size][ex~concurrency~thread-builder]     | {{#crate std}} | {{#categories concurrency=Concurrency}} |
