# Shared state

The Rust standard library provides smart pointer types, such as `Mutex<T>` and `Arc<T>`, that are safe to use in concurrent contexts.

## Mutex

Allow access to data from one thread at a time.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {

    // We wrap Mutex in Arc to allow for multiple owners.
    // Arc<T> is safe to use in concurrent situations.
    let counter = Arc::new(Mutex::new(0)); 
    let mut handles = vec![];

    for _ in 0..10 {
        // `clone` is somewhat a misnomer; it creates another pointer to the same Mutex, increasing the strong reference count.
        let counter = Arc::clone(&counter); 
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }  // releases the lock automatically when the MutexGuard goes out of scope.
        ); 
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

## Parking Lot

[Parking Lot]( https://crates.io/crates/parking_lot ) provides implementations of `Mutex`, `RwLock`, `Condvar` and `Once` that are smaller, faster and more flexible than those in the Rust standard library. It also provides a ReentrantMutex type.

`std::sync::Mutex` works fine, but Parking Lot is faster.

```rust,ignore
use parking_lot::Once;

static START: Once = Once::new();

// run a one-time initialization
START.call_once(|| {
    // run initialization here
});
```

```rust,ignore
use parking_lot::RwLock;

let lock = RwLock::new(5);

// many reader locks can be held at once
{
    let r1 = lock.read();
    let r2 = lock.read();
    assert_eq!(*r1, 5);
    assert_eq!(*r2, 5);
} // read locks are dropped at this point

// only one write lock may be held, however
{
    let mut w = lock.write();
    *w += 1;
    assert_eq!(*w, 6);
} // write lock is dropped here
```

## Crossbeam

[crossbeam](https://docs.rs/crossbeam/latest/crossbeam/)

```rust
use crossbeam_utils::atomic::AtomicCell;

let a = AtomicCell::new(7);
let v = a.into_inner();

assert_eq!(v, 7);
```
