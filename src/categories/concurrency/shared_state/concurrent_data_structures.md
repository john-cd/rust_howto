# Concurrent Data Structures

{{#include concurrent_data_structures.incl.md}}

## Dashmap

[![dashmap][dashmap-badge]][dashmap]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

[`Dashmap`][dashmap]⮳ is an implementation of a concurrent associative array/hashmap in Rust.

[`Dashmap`][dashmap]⮳ tries to be a direct replacement for `RwLock<HashMap<K, V>>`.

```rust,editable,noplayground
{{#include ../../../../deps/tests/dashmap.rs}}
```

## Bounded Multi-producer Multi-consumer Queue

[![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

```rust,editable,mdbook-runnable
{{#include ../../../../deps/tests/crossbeam_queue.rs}}
```

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
