// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates using `seize` for safe, concurrent memory reclamation.
//!
//! `seize` provides fast and efficient memory reclamation for concurrent
//! data structures. It solves the classic problem of safely freeing
//! shared memory when multiple threads may hold references to it.
//!
//! The key types are:
//! - [`Collector`]: tracks threads and manages reclamation epochs.
//! - `LocalGuard` (via `Collector::enter`): a per-thread guard that protects objects from being
//!   reclaimed while it is held.
//!
//! `seize` is primarily useful when building lock-free data structures
//! where multiple threads share ownership of heap-allocated nodes.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! seize = "0.5"
//! ```

use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;

use seize::Collector;
use seize::Guard as _;

/// Retires a shared object and reclaims it once no thread can access it.
fn main() {
    example();
}

/// Demonstrates guarding and retiring a shared pointer with a collector.
fn example() {
    // A `Collector` manages the access and retirement of concurrent objects.
    // It is typically stored alongside the data structure it protects.
    let collector = Collector::new();

    // Box a value and obtain a raw pointer that will be managed by `seize`.
    // `Box::into_raw` transfers ownership out of the `Box` without dropping
    // the pointee; `seize` will later reclaim and drop it at a safe time via
    // `collector.retire`.
    let ptr: *mut i32 = Box::into_raw(Box::new(100_i32));

    // Store the pointer in an atomic so it can be shared across threads.
    let shared = AtomicPtr::new(ptr);

    // `enter` marks the current thread as active and returns a guard. While
    // this guard is live, the collector will not reclaim protected objects.
    let guard = collector.enter();

    // `guard.protect` loads the pointer and keeps it valid for the guard's
    // lifetime. `Acquire` pairs with a writer's release operation.
    let raw = guard.protect(&shared, Ordering::Acquire);
    if !raw.is_null() {
        // SAFETY: `raw` is non-null and protected by `guard`.
        let value: i32 = unsafe { *raw };
        println!("Read value: {value}");
    }

    // Unpublish the pointer before retirement. A concurrent data structure
    // must make every reader use the same protection protocol.
    let retired = shared.swap(std::ptr::null_mut(), Ordering::AcqRel);
    assert_eq!(retired, ptr);

    drop(guard);

    // `reclaim::boxed` reconstructs the original Box and drops its `T` when
    // no active guard can still access the pointer.
    // SAFETY: the pointer is no longer published and this example owns it.
    unsafe { collector.retire(retired, seize::reclaim::boxed::<i32>) };

    println!("Value retired; memory will be reclaimed by the collector.");
}
// ANCHOR_END: example

/// Runs the example through its program entry point.
pub fn run() {
    example();
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the seize example through its entry point.
    #[test]
    fn test_seize_example_entry_point() {
        main();
    }

    /// A retired value is dropped when the collector is dropped.
    #[test]
    fn retired_value_is_reclaimed() {
        use std::sync::atomic::AtomicUsize;

        /// Shared `dropped` used by this module.
        static DROPPED: AtomicUsize = AtomicUsize::new(0);
        /// Data carried by tracked operations.
        struct Tracked(
            /// Value whose destructor activity is recorded by the example.
            i32,
        );
        impl Drop for Tracked {
            /// Releases the resources owned by this value.
            fn drop(&mut self) {
                DROPPED.fetch_add(1, Ordering::SeqCst);
            }
        }

        let collector = Collector::new();
        let ptr = Box::into_raw(Box::new(Tracked(5)));
        let shared = AtomicPtr::new(ptr);
        let guard = collector.enter();
        let raw = guard.protect(&shared, Ordering::Acquire);
        // SAFETY: `raw` is non-null and protected by `guard`.
        assert_eq!(unsafe { (*raw).0 }, 5);
        let retired = shared.swap(std::ptr::null_mut(), Ordering::AcqRel);
        assert_eq!(retired, ptr);
        drop(guard);
        // SAFETY: the pointer is no longer published and this test owns it.
        unsafe { collector.retire(retired, seize::reclaim::boxed::<Tracked>) };
        drop(collector);
        assert_eq!(DROPPED.load(Ordering::SeqCst), 1);
    }
}
