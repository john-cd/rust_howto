# Concurrency

This section covers concurrent programming, specifically parallel programming and async programming.

## Parallelism

- True simultaneous execution of multiple tasks on multiple cores or processors.
- Mechanism: uses operating system threads.
- Important for CPU-heavy computations.
- Often requires explicit management of threads and thread pools.
- Requires careful synchronization to prevent data races (using mechanisms like Mutexes or atomics).

- Overhead due to thread creation and switching.

Key constructs in Rust:

- Threads are independent units of execution that can be spawned using e.g. `std::thread::spawn`.
- Mutexes e.g. `std::sync::Mutex` protect shared data from race conditions.
- Channels e.g. `std::sync::mpsc` allow threads to communicate and exchange data.

Here are the topics we’ll cover:

- [Multithreading](concurrency/multithreading.md)
- [Message passing](concurrency/message_passing.md)
- [Shared-state concurrency](concurrency/shared_state.md)
- [Concurrent data structures](concurrency/shared_state/concurrent_data_structures.md)

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Spawn a short-lived thread][ex-crossbeam-spawn] | [![crossbeam-badge]][crossbeam] | [![cat-concurrency-badge]][cat-concurrency] |
| [Create a parallel data pipeline][ex-crossbeam-pipeline] | [![crossbeam-badge]][crossbeam] | [![cat-concurrency-badge]][cat-concurrency] |
| [Pass data between two threads][ex-crossbeam-spsc] | [![crossbeam-badge]][crossbeam] | [![cat-concurrency-badge]][cat-concurrency] |
| [Maintain global mutable state][ex-global-mut-state] | [![lazy-static-badge]][lazy-static] | [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Calculate SHA1 sum of *.iso files concurrently][ex-threadpool-walk] | [![threadpool-badge]][threadpool]  [![walkdir-badge]][walkdir]  [![num-cpus-badge]][num-cpus]  [![ring-badge]][ring] | [![cat-concurrency-badge]][cat-concurrency][![cat-filesystem-badge]][cat-filesystem] |
| [Draw fractal dispatching work to a thread pool][ex-threadpool-fractal] | [![threadpool-badge]][threadpool]  [![num-badge]][num]  [![num-cpus-badge]][num-cpus]  [![image-badge]][image] | [![cat-concurrency-badge]][cat-concurrency][![cat-science-badge]][cat-science][![cat-rendering-badge]][cat-rendering] |
| [Mutate the elements of an array in parallel][ex-rayon-iter-mut] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |
| [Test in parallel if any or all elements of a collection match a given predicate][ex-rayon-any-all] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |
| [Search items using given predicate in parallel][ex-rayon-parallel-search] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |
| [Sort a vector in parallel][ex-rayon-parallel-sort] | [![rayon-badge]][rayon]  [![rand-badge]][rand] | [![cat-concurrency-badge]][cat-concurrency] |
| [Map-reduce in parallel][ex-rayon-map-reduce] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |
| [Generate jpg thumbnails in parallel][ex-rayon-thumbnails] | [![rayon-badge]][rayon]  [![glob-badge]][glob]  [![image-badge]][image] | [![cat-concurrency-badge]][cat-concurrency][![cat-filesystem-badge]][cat-filesystem] |

## See Also

[![book-rust-concurrency-badge]][book-rust-concurrency]

[ex-crossbeam-spawn]: concurrency/threads.md#spawn-a-short-lived-thread
[ex-crossbeam-pipeline]: concurrency/threads.md#create-a-parallel-pipeline
[ex-crossbeam-spsc]: concurrency/threads.md#pass-data-between-two-threads
[ex-global-mut-state]: concurrency/threads.md#maintain-global-mutable-state
[ex-threadpool-walk]: concurrency/threads.md#calculate-sha256-sum-of-iso-files-concurrently
[ex-threadpool-fractal]: concurrency/threads.md#draw-fractal-dispatching-work-to-a-thread-pool
[ex-rayon-iter-mut]: concurrency/parallel.md#mutate-the-elements-of-an-array-in-parallel
[ex-rayon-any-all]: concurrency/parallel.md#test-in-parallel-if-any-or-all-elements-of-a-collection-match-a-given-predicate
[ex-rayon-parallel-search]: concurrency/parallel.md#search-items-using-given-predicate-in-parallel
[ex-rayon-parallel-sort]: concurrency/parallel.md#sort-a-vector-in-parallel
[ex-rayon-map-reduce]: concurrency/parallel.md#map-reduce-in-parallel
[ex-rayon-thumbnails]: concurrency/parallel.md#generate-jpg-thumbnails-in-parallel
{{#include refs/link-refs.md}}
