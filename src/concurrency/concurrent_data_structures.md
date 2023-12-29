# Concurrent Data Structures

## Dashmap

[Dashmap]( https://docs.rs/dashmap/5.3.3/dashmap/struct.DashMap.html# ) is an implementation of a concurrent associative array/hashmap in Rust.

DashMap tries to be a direct replacement for `RwLock<HashMap<K, V>>`.

```rust,editable,ignore
{{#include ../../deps/examples/dashmap.rs}}
```

## Bounded Multi-producer Multi-consumer Queue

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/crossbeam_queue.rs}}
```
