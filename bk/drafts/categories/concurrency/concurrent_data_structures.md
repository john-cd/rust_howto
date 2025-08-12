# Concurrent Data Structures

{{#include concurrent_data_structures.incl.md}}

Refer to the [comparative benchmarks of concurrent HashMaps][conc-map-bench~github]↗.

## `dashmap` {#dashmap}

[![dashmap][c~dashmap~docs~badge]][c~dashmap~docs] [![dashmap~crates.io][c~dashmap~crates.io~badge]][c~dashmap~crates.io] [![dashmap~github][c~dashmap~github~badge]][c~dashmap~github] [![dashmap~lib.rs][c~dashmap~lib.rs~badge]][c~dashmap~lib.rs]{{hi:dashmap}}{{hi:Concurrent}}{{hi:Hashmap}}{{hi:Atomic}}[![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

[`dashmap`][c~dashmap~docs]{{hi:dashmap}}↗ is a fast concurrent `HashMap` i.e. a concurrent associative array.{{hi:Concurrent associative array}}

[`dashmap`][c~dashmap~docs]{{hi:dashmap}}↗ tries to be a direct replacement for `RwLock<HashMap<K, V>>`.

It allows multiple threads to concurrently read and write to the map with minimal contention, using a technique called "shard-based" or "bucket-based" concurrency. This makes [`dashmap`][c~dashmap~docs]↗{{hi:dashmap}} a good choice when you need a hash map that can be accessed frequently by multiple threads without significant performance bottlenecks.

```rust,editable,noplayground
{{#include ../../../crates/cats/concurrency/examples/concurrent_data_structures/dashmap.rs:example}}
```

## Bounded Multi-producer Multi-consumer Queue {#crossbeam-queue}

[![crossbeam-queue~website][c~crossbeam-queue~website~badge]][c~crossbeam-queue~website] [![crossbeam-queue][c~crossbeam-queue~docs~badge]][c~crossbeam-queue~docs] [![crossbeam-queue~crates.io][c~crossbeam-queue~crates.io~badge]][c~crossbeam-queue~crates.io] [![crossbeam-queue~github][c~crossbeam-queue~github~badge]][c~crossbeam-queue~github] [![crossbeam-queue~lib.rs][c~crossbeam-queue~lib.rs~badge]][c~crossbeam-queue~lib.rs]{{hi:crossbeam-queue}}{{hi:Queue}}{{hi:Mpmc}}{{hi:Lock-free}}{{hi:Producer}}{{hi:Consumer}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`crossbeam-queue`][c~crossbeam-queue~docs]↗{{hi:crossbeam-queue}} provides various concurrent queue implementations in Rust, designed for efficient and safe communication between threads. It offers different queue types optimized for various use cases, including single-producer/single-consumer, multi-producer/multi-consumer, and bounded/unbounded queues. These queues are essential for building concurrent data structures and message-passing systems, enabling threads to exchange data without race conditions or memory safety issues.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/concurrent_data_structures/crossbeam_queue.rs:example}}
```

## `flurry` {#flurry}

[![flurry][c~flurry~docs~badge]][c~flurry~docs] [![flurry~crates.io][c~flurry~crates.io~badge]][c~flurry~crates.io] [![flurry~github][c~flurry~github~badge]][c~flurry~github] [![flurry~lib.rs][c~flurry~lib.rs~badge]][c~flurry~lib.rs]{{hi:flurry}}{{hi:Map}}{{hi:Concurrent}}{{hi:Hashmap}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

[`flurry`][c~flurry~docs]↗{{hi:flurry}} is a concurrent hash table designed for high performance. It allows fully concurrent reads and highly concurrent updates. Its main type is functionally very similar to [`std::collections::HashMap`][c~std::collections::HashMap~docs]↗{{hi:std::collections::HashMap}}. Its implementation is closely based on Java's `java.util.concurrent.ConcurrentHashMap`. Even though all operations on the map are thread-safe and operate on shared references, retrieval operations do not entail locking, and there is not any support for locking the entire table in a way that prevents all access ([doc][c~flurry~docs]↗).

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/concurrent_data_structures/flurry.rs:example}}
```

## `papaya` {#papaya}

[![papaya][c~papaya~docs~badge]][c~papaya~docs] [![papaya~crates.io][c~papaya~crates.io~badge]][c~papaya~crates.io] [![papaya~github][c~papaya~github~badge]][c~papaya~github] [![papaya~lib.rs][c~papaya~lib.rs~badge]][c~papaya~lib.rs]{{hi:papaya}}{{hi:Atomic}}{{hi:Concurrent}}{{hi:Hashmap}}{{hi:Lock-free}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

[`papaya`][c~papaya~docs]↗{{hi:papaya}} offers a fast and ergonomic concurrent hash-table for read-heavy workloads.

- Ergonomic lock-free API - no more deadlocks!
- Powerful atomic operations.
- Seamless usage in [async][p~async] contexts.
- Extremely scalable, low-latency reads (see [performance][p~performance]).
- Predictable latency across all operations.
- Efficient memory usage, with garbage collection powered by [`seize`][c~seize~docs]↗{{hi:seize}}.
([doc][c~papaya~docs]↗)

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/concurrent_data_structures/papaya.rs:example}}
```

## Related Topics {#related-topics}

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
[concurrent_data_structures: finish](https://github.com/john-cd/rust_howto/issues/258)
</div>
