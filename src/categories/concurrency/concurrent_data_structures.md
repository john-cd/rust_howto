# Concurrent Data Structures

{{#include concurrent_data_structures.incl.md}}

## `dashmap` {#dashmap}

[![dashmap][c-dashmap-badge]][c-dashmap] [![dashmap-crates.io][c-dashmap-crates.io-badge]][c-dashmap-crates.io] [![dashmap-github][c-dashmap-github-badge]][c-dashmap-github] [![dashmap-lib.rs][c-dashmap-lib.rs-badge]][c-dashmap-lib.rs]{{hi:dashmap}}{{hi:Concurrent}}{{hi:Hashmap}}{{hi:Atomic}}[![cat-algorithms][cat-algorithms-badge]][cat-algorithms]{{hi:Algorithms}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}

Fast concurrent HashMap for Rust.

[`dashmap`][c-dashmap]{{hi:dashmap}}⮳ is an implementation of a concurrent associative array{{hi:Concurrent associative array}} / hashmap in Rust. [`dashmap`][c-dashmap]{{hi:dashmap}}⮳ tries to be a direct replacement for `RwLock<HashMap<K, V>>`.

```rust,editable,noplayground
{{#include ../../../deps/tests/categories/concurrency/dashmap.rs:example}}
```

## Bounded multi-producer multi-consumer queue {#crossbeam-queue}

[![crossbeam-queue-website][c-crossbeam_queue-website-badge]][c-crossbeam_queue-website] [![crossbeam-queue][c-crossbeam_queue-badge]][c-crossbeam_queue] [![crossbeam-queue-crates.io][c-crossbeam_queue-crates.io-badge]][c-crossbeam_queue-crates.io] [![crossbeam-queue-github][c-crossbeam_queue-github-badge]][c-crossbeam_queue-github] [![crossbeam-queue-lib.rs][c-crossbeam_queue-lib.rs-badge]][c-crossbeam_queue-lib.rs]{{hi:crossbeam-queue}}{{hi:Queue}}{{hi:Mpmc}}{{hi:Lock-free}}{{hi:Producer}}{{hi:Consumer}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Concurrent queues.

```rust,editable
{{#include ../../../deps/tests/categories/concurrency/crossbeam_queue.rs:example}}
```

## `flurry` {#flurry}

[![flurry][c-flurry-badge]][c-flurry] [![flurry-crates.io][c-flurry-crates.io-badge]][c-flurry-crates.io] [![flurry-github][c-flurry-github-badge]][c-flurry-github] [![flurry-lib.rs][c-flurry-lib.rs-badge]][c-flurry-lib.rs]{{hi:flurry}}{{hi:Map}}{{hi:Concurrent}}{{hi:Hashmap}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}

`flurry` is particularly good for read-heavy workloads.

Refer to the [comparative benchmarks of concurrent HashMaps][conc-map-bench]⮳ as well.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P1 add flurry example
</div>
