# Concurrent Data Structures

{{#include concurrent_data_structures.incl.md}}

## Dashmap

[![dashmap][c-dashmap-badge]][c-dashmap]{{hi:dashmap}}  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

[`dashmap`][c-dashmap]{{hi:dashmap}}⮳ is an implementation of a concurrent associative array{{hi:Concurrent associative array}} / hashmap in Rust.

[`dashmap`][c-dashmap]{{hi:dashmap}}⮳ tries to be a direct replacement for `RwLock<HashMap<K, V>>`.

```rust,noplayground
{{#include ../../../../deps/tests/dashmap.rs}}
```

## Bounded Multi-producer Multi-consumer Queue

[![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

```rust,mdbook-runnable
{{#include ../../../../deps/tests/crossbeam_queue.rs}}
```

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
TODO: add crate badges
</div>
