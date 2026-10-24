<!-- TODO Reviewed ??-??-2026 -->

Use the Crossbeam crates when an application needs MPMC channels, channel selection, lock-free queues, or concurrent utility types. `std::thread::scope` covers scoped thread creation in current Rust.

| Recipe                                                                           | Crates                                                                           | Categories                                                   |
| -------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| [Select Across Several Channels][ex~concurrency~select-channels]                 | [![crossbeam-channel][c~crossbeam-channel~docs~badge]][c~crossbeam-channel~docs] | [![cat~concurrency][cat~concurrency~badge]][cat~concurrency] |
| [Distribute Work with a Bounded MPMC Pipeline][ex~concurrency~parallel-pipeline] | [![crossbeam-channel][c~crossbeam-channel~docs~badge]][c~crossbeam-channel~docs] | [![cat~concurrency][cat~concurrency~badge]][cat~concurrency] |
| [Concurrent Queues][ex~concurrency~crossbeam-queue]                              | [![crossbeam-queue][c~crossbeam-queue~docs~badge]][c~crossbeam-queue~docs]       | [![cat~concurrency][cat~concurrency~badge]][cat~concurrency] |
| [AtomicCell][ex~concurrency~crossbeam-atomics]                                   | [![crossbeam-utils][c~crossbeam-utils~docs~badge]][c~crossbeam-utils~docs]       | [![cat~concurrency][cat~concurrency~badge]][cat~concurrency] |
