# Concurrent Data Structures

## Dashmap

[![dashmap-badge]][dashmap]

`Dashmap` is an implementation of a concurrent associative array/hashmap in Rust.

`DashMap` tries to be a direct replacement for `RwLock<HashMap<K, V>>`.

```rust,editable,ignore,noplayground
{{#include ../../deps/examples/dashmap.rs}}
```

## Bounded Multi-producer Multi-consumer Queue

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/crossbeam_queue.rs}}
```

{{#include ../refs/link-refs.md}}
