# Concurrent Data Structures

## Dashmap

[![dashmap-badge]][dashmap]

`Dashmap` is an implementation of a concurrent associative array/hashmap in Rust.

`DashMap` tries to be a direct replacement for `RwLock<HashMap<K, V>>`.

```rust,editable,noplayground
{{#include ../../../deps/tests/dashmap.rs}}
```

## Bounded Multi-producer Multi-consumer Queue

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/crossbeam_queue.rs}}
```

{{#include ../../refs/link-refs.md}}
