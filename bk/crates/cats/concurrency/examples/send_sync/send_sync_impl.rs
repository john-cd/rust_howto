// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Simplified implementation of the `Arc` smart pointer.
//!
//! Invoking `clone` on `Arc` produces a new `Arc` instance, which points to the
//! same allocation on the heap as the source `Arc`, while increasing a
//! reference count.
//!
//! This implementation is not intended for production use. Cloning panics
//! rather than allowing the strong count to overflow.

/// A simplified `Arc` that implements `Send` and `Sync`.
mod my {

    use std::marker::PhantomData;
    use std::marker::Send;
    use std::marker::Sync;
    use std::ops::Deref;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    /// Private inner data structure.
    /// It holds the actual data and the reference count.
    /// One instance is shared by / pointed to by multiple `Arc` clones.
    struct ArcInner<T> {
        /// The number of references.
        count: AtomicUsize,
        /// The shared data.
        data: T,
    }

    /// A simplified implementation of `Arc`.
    /// The fields are private and unreachable outside of the module.
    pub struct Arc<T> {
        // Pointer to the inner data structure.
        /// A pointer to the shared data and reference count.
        ptr: *mut ArcInner<T>,
        // We want `Arc` to behave like a `ArcInner<T>`, despite the fact that
        // it does not own one, thus we use `PhantomData`.
        /// A marker that tells the compiler that `Arc` behaves like an owner
        /// of the inner data.
        phantom: PhantomData<ArcInner<T>>,
    }

    // SAFETY: Every instance owns one strong count on a valid allocation.
    // The count uses atomics, and sharing `T` requires both `Send` and `Sync`.
    unsafe impl<T: Sync + Send> Send for Arc<T> {}

    unsafe impl<T: Sync + Send> Sync for Arc<T> {}

    impl<T> Arc<T> {
        /// Creates a new `Arc` instance.
        ///
        /// # Arguments
        ///
        /// * `data` - The data to be shared.
        pub fn new(data: T) -> Self {
            // Store the allocation's ownership in the raw pointer. The final
            // Arc drop reconstructs this Box and frees the allocation.
            let inner = Box::new(ArcInner {
                count: AtomicUsize::new(1),
                data,
            });
            let ptr = Box::into_raw(inner);
            println!("Created `Arc` with reference count 1.");
            // SAFETY: `ptr` points to the `ArcInner` we just allocated.
            unsafe { Self::from_inner(ptr) }
        }

        /// Returns a reference to the inner data.
        ///
        /// The allocation stays live while this `Arc` owns one strong count.
        fn inner(&self) -> &ArcInner<T> {
            // SAFETY: `Arc::new` creates a valid allocation, and cloning only
            // copies the pointer while incrementing its strong count.
            unsafe {
                self.ptr
                    .as_ref()
                    .expect("the send sync impl example invariant should hold")
            }
        }

        /// Creates a new `Arc` from a raw pointer to an `ArcInner`.
        ///
        /// # Safety
        ///
        /// `ptr` must be non-null, aligned, point to a valid `ArcInner<T>`,
        /// and represent one strong reference that this value will release.
        unsafe fn from_inner(ptr: *mut ArcInner<T>) -> Self {
            Self {
                ptr,
                phantom: PhantomData,
            }
        }
    }

    /// Implements the `Clone` trait for `Arc`.
    ///
    /// This increments the reference count.
    impl<T> Clone for Arc<T> {
        fn clone(&self) -> Self {
            // Increase the count only if it will not wrap and invalidate the
            // ownership guarantee. `fetch_update` leaves the count unchanged
            // when `checked_add` fails.
            let previous_count = self
                .inner()
                .count
                .try_update(Ordering::Relaxed, Ordering::Relaxed, |count| {
                    count.checked_add(1)
                })
                .unwrap_or_else(|_| {
                    panic!("Arc strong reference count overflow")
                });
            println!(
                "Increased the reference count; previous value: {previous_count}"
            );
            Self {
                ptr: self.ptr,
                phantom: PhantomData,
            }
        }
    }

    // Implement `Deref` to make `Arc` a smart pointer.
    impl<T> Deref for Arc<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            // SAFETY: this `Arc` owns a live strong count, so its allocation
            // and initialized data remain valid for the returned borrow.
            unsafe { &(*self.ptr).data }
        }
    }

    /// Implements the `Drop` trait for `Arc`.
    ///
    /// This decrements the reference count and deallocates the data when the
    /// count reaches zero.
    impl<T> Drop for Arc<T> {
        fn drop(&mut self) {
            // Subtracts from the current value, returning the previous value.
            // `count` is atomic, so this is thread-safe.
            let previous_count =
                self.inner().count.fetch_sub(1, Ordering::Release);
            if previous_count != 1 {
                println!(
                    "Decreased reference count; previous count: {previous_count}"
                );
                return;
            }
            // Pair this acquire fence with every Release decrement so the
            // final owner observes accesses made before other owners dropped.
            std::sync::atomic::fence(Ordering::Acquire);

            // SAFETY: the count reached zero, so this is the unique final
            // owner. `Box::from_raw` drops `T` once and frees the allocation.
            unsafe { drop(Box::from_raw(self.ptr)) };

            println!("`Arc` data fully dropped!");
        }
    }

    #[cfg(test)]
    mod tests {
        use std::panic::AssertUnwindSafe;
        use std::panic::catch_unwind;
        use std::sync::Arc as StdArc;
        use std::sync::atomic::AtomicUsize;
        use std::sync::atomic::Ordering;

        use super::Arc;

        /// Tracks whether the shared value is destroyed.
        struct DropCounter(StdArc<AtomicUsize>);

        impl Drop for DropCounter {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::Relaxed);
            }
        }

        /// Verifies the last owner can drop the value on another thread once.
        #[test]
        fn final_owner_drops_value_once_on_another_thread() {
            let drops = StdArc::new(AtomicUsize::new(0));
            let value = Arc::new(DropCounter(StdArc::clone(&drops)));
            let worker_value = value.clone();
            drop(value);

            std::thread::spawn(move || drop(worker_value))
                .join()
                .expect("the final owner thread should finish");

            assert_eq!(drops.load(Ordering::Relaxed), 1);
        }

        /// Verifies that a failed clone leaves an overflowing count unchanged.
        #[test]
        fn clone_panics_before_strong_count_overflow() {
            let value = Arc::new(());
            value.inner().count.store(usize::MAX, Ordering::Relaxed);

            let clone_result = catch_unwind(AssertUnwindSafe(|| value.clone()));

            assert!(clone_result.is_err());
            assert_eq!(value.inner().count.load(Ordering::Relaxed), usize::MAX);

            // Restore the private test fixture's count so its own drop frees
            // the allocation instead of modeling an impossible live state.
            value.inner().count.store(1, Ordering::Relaxed);
        }
    }
}

/// A struct that prints when dropped, for demonstration purposes.
#[derive(Debug)]
struct Droppable(i32);

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("`Droppable` drop called - its value was {}", self.0);
    }
}

/// Demonstrate the usage of `my::Arc`.
///
/// This function creates an `my::Arc` instance, spawns multiple threads that
/// access the shared data, and then waits for all threads to finish.
fn main() {
    let value = Droppable(42);

    let reference = &value;
    println!("The address of `value` is {reference:p}");

    let my_arc = my::Arc::new(value);

    // Spawn multiple threads to demonstrate that `Arc` is `Send` and `Sync`.
    let handles: Vec<_> = (0..3)
        .map(|i| {
            // Clone the `Arc` (increasing the reference count),
            // move the clone into the thread.
            let my_clone = my_arc.clone();
            std::thread::spawn(move || {
                println!("Data: {:?}", *my_clone);
                assert_eq!(my_clone.0, 42);
                println!("`my_clone` in thread {i} will drop next.");
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("worker thread should not panic");
    } // At this point, all clones have been dropped and only `my_arc` remains.

    println!("`my_arc` will drop next.");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the send sync impl example through its entry point.
    #[test]
    fn test_send_sync_impl_example_entry_point() {
        main();
    }
}
// Miri: the `unsafe` code above relies on the raw `ArcInner` pointer staying
// valid until the strong count reaches zero, then being reclaimed exactly
// once. From `bk/crates`, run `cargo +nightly miri test -p concurrency
// --example send_sync final_owner_drops_value_once_on_another_thread`. Miri can
// catch invalid pointer use and report the leaked allocation; it does not model
// weak-memory reorderings, which need a faithful Loom model.
