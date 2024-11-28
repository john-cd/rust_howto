# Concurrent Data Structures

{{#include concurrent_data_structures.incl.md}}

## Dashmap {#dashmap}

[![dashmap][c-dashmap-badge]][c-dashmap]{{hi:dashmap}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

[`dashmap`][c-dashmap]{{hi:dashmap}}⮳ is an implementation of a concurrent associative array{{hi:Concurrent associative array}} / hashmap in Rust.

[`dashmap`][c-dashmap]{{hi:dashmap}}⮳ tries to be a direct replacement for `RwLock<HashMap<K, V>>`.

```rust,editable,noplayground
{{#include ../../../../deps/tests/cats/concurrency/dashmap.rs:example}}
```

## Bounded Multi-producer Multi-consumer Queue {#crossbeam-queue}

[![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

```rust,editable
{{#include ../../../../deps/tests/cats/concurrency/crossbeam_queue.rs:example}}
```

## flurry {#flurry}

[![flurry][c-flurry-badge]][c-flurry]{{hi:flurry}}
[![flurry-crates.io][c-flurry-crates.io-badge]][c-flurry-crates.io]
[![flurry-github][c-flurry-github-badge]][c-flurry-github]
[![flurry-lib.rs][c-flurry-lib.rs-badge]][c-flurry-lib.rs]

Particularly good for read-heavy workloads.

[conc map bench][conc-map-bench] comparative benchmarks of concurrent HashMaps.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
TODO P1 add crate badges
TODO P1 add flurry example
</div>
