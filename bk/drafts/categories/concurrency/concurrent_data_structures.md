<!-- TODO Reviewed ??-??-2026 -->

# Concurrent Data Structures

{{#include concurrent_data_structures.incl.md}}

Refer to the [comparative benchmarks of concurrent HashMaps][conc-map-bench~repo]↗.

| Crate                                        | Structure                       | Lock-free?                                                   | Notes                                                                                   |
| -------------------------------------------- | ------------------------------- | ------------------------------------------------------------ | --------------------------------------------------------------------------------------- |
| [`dashmap`][c~dashmap~docs]↗                 | Sharded hash map                | No; uses shard locks                                         | Familiar map API. Keep entry guards short-lived.                                        |
| [`papaya`][c~papaya~docs]↗                   | Concurrent hash table           | Yes; lock-free map API                                       | Pin the map for access; pins also protect values from reclamation.                      |
| [`scc`][c~scc~docs]↗                         | Concurrent hash map and indexes | Mixed; entry access uses bucket locks, resizing is lock-free | Provides sync and async interfaces.                                                     |
| [`flurry`][c~flurry~docs]↗                   | Concurrent hash table           | Reads do not lock; updates are synchronized                  | Access methods use a guard for safe reclamation.                                        |
| [`crossbeam-queue`][c~crossbeam-queue~docs]↗ | Bounded MPMC queue              | Yes                                                          | `ArrayQueue` has a fixed capacity and reports when it is full.                          |
| [`arc-swap`][c~arc-swap~docs]↗               | Atomic shared-pointer slot      | Readers are lock-free; writers are lock-free                 | Useful for read-mostly configuration snapshots; see [ArcSwap][ex~concurrency~arc-swap]. |

## `dashmap` {#dashmap}

{{#badges dashmap algorithms=Algorithms concurrency=Concurrency data-structures="Data structures"}}{{hi:Concurrent}}{{hi:Hashmap}}{{hi:Atomic}}

[`dashmap`][c~dashmap~docs]↗{{hi:dashmap}} is a fast concurrent `HashMap` i.e. a concurrent associative array.{{hi:Concurrent associative array}}

[`dashmap`][c~dashmap~docs]↗{{hi:dashmap}} tries to be a direct replacement for `RwLock<HashMap<K, V>>`.

It allows multiple threads to concurrently read and write to the map with minimal contention, using a technique called "shard-based" or "bucket-based" concurrency. This makes [`dashmap`][c~dashmap~docs]↗{{hi:dashmap}} a good choice for a hash map that can be accessed frequently by multiple threads without significant performance bottlenecks.

An entry guard returned by `get` or `get_mut` holds a shard lock. Drop guards promptly; holding one across `.await` can block executor progress, and re-entering the same map while a guard is held can deadlock.

```rust,editable,noplayground
{{#include ../../../crates/cats/concurrency/examples/concurrent_data_structures/dashmap.rs:example}}
```

## Bounded Multi-producer Multi-consumer Queue {#crossbeam-queue}

[![crossbeam-queue~website][c~crossbeam-queue~website~badge]][c~crossbeam-queue~website] [![crossbeam-queue][c~crossbeam-queue~docs~badge]][c~crossbeam-queue~docs] [![crossbeam-queue~crates.io][c~crossbeam-queue~crates.io~badge]][c~crossbeam-queue~crates.io] [![crossbeam-queue~repo][c~crossbeam-queue~repo~badge]][c~crossbeam-queue~repo] [![crossbeam-queue~lib.rs][c~crossbeam-queue~lib.rs~badge]][c~crossbeam-queue~lib.rs]{{hi:crossbeam-queue}}{{hi:Queue}}{{hi:Mpmc}}{{hi:Lock-free}}{{hi:Producer}}{{hi:Consumer}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`crossbeam-queue`][c~crossbeam-queue~docs]↗{{hi:crossbeam-queue}} provides various concurrent queue implementations in Rust, designed for efficient and safe communication between threads. It offers different queue types optimized for various use cases, including single-producer/single-consumer, multi-producer/multi-consumer, and bounded/unbounded queues. These queues are essential for building concurrent data structures and message-passing systems, enabling threads to exchange data without race conditions or memory safety issues.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/concurrent_data_structures/crossbeam_queue.rs:example}}
```

## `flurry` {#flurry}

{{#badges flurry concurrency=Concurrency data-structures="Data structures"}}{{hi:Map}}{{hi:Concurrent}}{{hi:Hashmap}}

[`flurry`][c~flurry~docs]↗{{hi:flurry}} is a concurrent hash table designed for high performance. It allows fully concurrent reads and highly concurrent updates. Its main type is functionally very similar to [`std::collections::HashMap`][c~std::collections::HashMap~docs]↗{{hi:std::collections::HashMap}}. Its implementation is closely based on Java's `java.util.concurrent.ConcurrentHashMap`. Even though all operations on the map are thread-safe and operate on shared references, retrieval operations do not entail locking, and there is not any support for locking the entire table in a way that prevents all access ([doc][c~flurry~docs]↗).

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/concurrent_data_structures/flurry.rs:example}}
```

## `papaya` {#papaya}

{{#badges papaya algorithms=Algorithms concurrency=Concurrency data-structures="Data structures"}}{{hi:Atomic}}{{hi:Concurrent}}{{hi:Hashmap}}{{hi:Lock-free}}

[`papaya`][c~papaya~docs]↗{{hi:papaya}} offers a fast and ergonomic concurrent hash-table for read-heavy workloads.

- Ergonomic lock-free API - no more deadlocks!
- Powerful atomic operations.
- Seamless usage in [async][p~async] contexts.
- Extremely scalable, low-latency reads (see [performance][p~performance]).
- Predictable latency across all operations.
- Efficient memory usage, with garbage collection powered by [`seize`][c~seize~docs]↗{{hi:seize}}.
  ([doc][c~papaya~docs]↗).

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/concurrent_data_structures/papaya.rs:example}}
```

For applications that need a synchronous or asynchronous concurrent map with bucket-level locking, consider [`scc`][c~scc~docs]↗{{hi:scc}}. Its map uses read-write locks for entry access while resizing proceeds lock-free.

## Related Topics {#related-topics .skip}

- [[async | Async]].
- [[async-channels | Async Channels]].
- [[data-structures | Data Structures]].
- [[global_static | Global Static]].
- [[rust-patterns | Rust Patterns]].
- [[send_sync | Send and Sync]].
- [[shared_state | Shared State]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
