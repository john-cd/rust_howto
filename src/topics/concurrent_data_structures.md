# Concurrent Data Structures

## Dashmap

[Dashmap]( https://docs.rs/dashmap/5.3.3/dashmap/struct.DashMap.html# ) is an implementation of a concurrent associative array/hashmap in Rust.

DashMap tries to be very simple to use and to be a direct replacement for `RwLock<HashMap<K, V>>`.

## bounded multi-producer multi-consumer queue

```rust,ignore
use crossbeam_queue::ArrayQueue;

let q = ArrayQueue::new(2);

assert_eq!(q.push('a'), Ok(()));
assert_eq!(q.push('b'), Ok(()));
assert_eq!(q.push('c'), Err('c'));
assert_eq!(q.pop(), Some('a'));
```
