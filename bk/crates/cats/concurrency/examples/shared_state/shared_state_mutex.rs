#![allow(dead_code)]
// ANCHOR: example
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

/// Demonstrates using a `Mutex` to safely share mutable state between threads.
fn main() {
    // We wrap `Mutex` in `Arc` to allow for multiple owners.
    // `Mutex<T>` provides mutual exclusion, ensuring only one thread can access
    // the data at a time. `Arc<T>` is safe to use in concurrent situations.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // `clone` is somewhat a misnomer; it creates another pointer to the
        // same `Mutex`, increasing the (strong) reference count.
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(
            move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }, /* Releases the lock automatically when the MutexGuard
                * goes out of scope. */
        );
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
