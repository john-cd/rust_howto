#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates the use of atomic types.
//!
//! This example uses `AtomicUsize` to maintain a global counter of threads.
//! Atomic operations ensure that modifications to the counter are
//! thread-safe.
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

// Declare a static atomic counter.
// Atomic variables are safe to share between threads (they implement `Sync`).
static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);

fn main() {
    // `fetch_add` atomically increments the counter and returns the previous
    // value.
    // Each atomic access takes an `Ordering`. `Ordering::SeqCst` ensures
    // sequential consistency, which is the strongest, safest, but slowest
    // memory ordering. Technically, a weaker `Ordering` would work here.
    // Read more about Ordering in the [Nomicon](https://doc.rust-lang.org/nomicon/atomics.html).
    let old_thread_count = GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
    println!("Threads: {}", old_thread_count + 1);

    // Create multiple threads.
    std::thread::scope(|s| {
        for _ in 0..5 {
            s.spawn(|| {
                // Increment the counter for each new thread.
                GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
            });
        }
    });
    // The scope guarantees all threads will be joined at the end of the scope.

    // Get the final thread count.
    let final_thread_count = GLOBAL_THREAD_COUNT.load(Ordering::SeqCst);
    println!("Final threads: {final_thread_count}");
    assert_eq!(final_thread_count, 6);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
