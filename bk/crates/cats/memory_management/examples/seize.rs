// ANCHOR: example
//! Demonstrates using `seize` for safe, concurrent memory reclamation.
//!
//! `seize` provides fast and efficient memory reclamation for concurrent
//! data structures. It solves the classic problem of safely freeing
//! shared memory when multiple threads may hold references to it.
//!
//! The key types are:
//! - [`Collector`]: tracks threads and manages reclamation epochs.
//! - [`LocalGuard`] (via `Collector::enter`): a per-thread guard that protects
//!   objects from being reclaimed while it is held.
//!
//! `seize` is primarily useful when building lock-free data structures
//! where multiple threads share ownership of heap-allocated nodes.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! seize = "0.5"
//! ```

use std::sync::Arc;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;

use seize::Collector;
use seize::Guard;

fn main() {
    // A `Collector` manages the access and retirement of concurrent objects.
    // It is typically stored alongside the data structure it protects.
    let collector = Arc::new(Collector::new());

    // Box a value and obtain a raw pointer that will be managed by `seize`.
    // `Box::into_raw` transfers ownership out of the `Box` without dropping
    // the pointee; `seize` will later reclaim and drop it at a safe time via
    // `collector.retire`.
    let ptr: *mut i32 = Box::into_raw(Box::new(100_i32));

    // Store the pointer in an atomic so it can be shared across threads.
    let shared = Arc::new(AtomicPtr::new(ptr));

    // --- Simulated reader ---
    {
        // `enter` marks the current thread as active and returns a guard.
        // While this guard is live, no managed object can be reclaimed.
        let guard = collector.enter();

        // `guard.protect` performs a safe atomic load: the loaded pointer
        // is guaranteed to remain valid for the lifetime of `guard`.
        // `Acquire` pairs with the `Release` store done by the writing
        // thread, ensuring all prior writes to the pointed-to value are
        // visible to this thread.
        let raw = guard.protect(&shared, Ordering::Acquire);

        if !raw.is_null() {
            // Safety: `raw` is non-null and protected by `guard`.
            let value: i32 = unsafe { *raw };
            println!("Read value: {value}");
        }

        // `guard` is dropped here; the thread is marked inactive.
    }

    // --- Reclamation ---
    // After all guards that could have seen this pointer are dropped,
    // retire it so the collector can free the memory when it is safe.
    //
    // `seize::reclaim::boxed` calls `Box::from_raw` and drops the original
    // `T`, so any destructor runs at reclamation time.
    //
    // Safety: `ptr` is no longer accessible to any other thread.
    unsafe {
        collector.retire(ptr, seize::reclaim::boxed::<i32>);
    }

    println!("Value retired; memory will be reclaimed by the collector.");
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
