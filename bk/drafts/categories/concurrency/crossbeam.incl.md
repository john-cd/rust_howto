<!-- TODO Reviewed ??-??-2026 -->

Use the Crossbeam crates when an application needs MPMC channels, channel selection, lock-free queues, or concurrent utility types. `std::thread::scope` covers scoped thread creation in current Rust.

| Recipe                                                                           | Crates                       | Categories                              |
| -------------------------------------------------------------------------------- | ---------------------------- | --------------------------------------- |
| [Select Across Several Channels][ex~concurrency~select-channels]                 | {{#crate crossbeam-channel}} | {{#categories concurrency=Concurrency}} |
| [Distribute Work with a Bounded MPMC Pipeline][ex~concurrency~parallel-pipeline] | {{#crate crossbeam-channel}} | {{#categories concurrency=Concurrency}} |
| [Concurrent Queues][ex~concurrency~crossbeam-queue]                              | {{#crate crossbeam-queue}}   | {{#categories concurrency=Concurrency}} |
| [AtomicCell][ex~concurrency~crossbeam-atomics]                                   | {{#crate crossbeam-utils}}   | {{#categories concurrency=Concurrency}} |
