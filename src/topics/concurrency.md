# Concurrency 


## Thread-based

```rust
let handle = thread::spawn(|| {
    for i in 1..10 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
});

// more stufff

handle.join().unwrap(); // wait for spawned thread
```

When the main thread of a Rust program completes, all spawned threads are shut down, whether or not they have finished running. 

## Message passing and shared state

The Rust standard library provides channels for message passing and smart pointer types, such as Mutex<T> and Arc<T>, that are safe to use in concurrent contexts. 


## See Also

[Concurrency]( https://doc.rust-lang.org/book/ch16-00-concurrency.html )

[Message Passing]( https://doc.rust-lang.org/book/ch16-02-message-passing.html )

[crossbeam](https://docs.rs/crossbeam/latest/crossbeam/)

[Parking Lot]( https://crates.io/crates/parking_lot ): more compact and efficient implementations of the standard synchronization primitives.

[Dashmap]( https://docs.rs/dashmap/5.3.3/dashmap/struct.DashMap.html# )

[Rayon]( https://github.com/rayon-rs/rayon ) for parallel data processing
