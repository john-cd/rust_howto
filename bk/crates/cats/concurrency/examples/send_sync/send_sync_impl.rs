// ANCHOR: example
//! Simplified implementation of the `Arc` smart pointer.
//!
//! Invoking `clone` on `Arc` produces a new `Arc` instance, which points to the
//! same allocation on the heap as the source `Arc`, while increasing a
//! reference count.
//!
//! This implementation is not intended for production use.
#![allow(dead_code)]

mod my {

    use std::marker::PhantomData;
    use std::marker::Send;
    use std::marker::Sync;
    use std::mem::ManuallyDrop;
    use std::ops::Deref;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering::Relaxed;
    use std::sync::atomic::Ordering::Release;

    /// Private inner data structure.
    /// It holds the actual data and the reference count.
    /// One instance is shared by / pointed to by multiple `Arc` clones.
    struct ArcInner<T> {
        count: AtomicUsize,
        data: T,
    }

    // `ArcInner` should be `Send` and `Sync` if the underlying data is.
    unsafe impl<T: Sync + Send> Send for ArcInner<T> {}
    unsafe impl<T: Sync + Send> Sync for ArcInner<T> {}

    /// A simplified implementation of `Arc`.
    /// The fields are private and unreachable outside of the module.
    pub struct Arc<T> {
        // Pointer to the inner data structure.
        ptr: *mut ArcInner<T>,
        // We want `Arc` to behave like a `ArcInner<T>`, despite the fact that
        // it does not own one, thus we use `PhantomData`.
        phantom: PhantomData<ArcInner<T>>,
    }

    // SAFETY: By implementing `Send` for `Arc`, we declare it can be sent
    // across threads safely. By implementing `Sync`, we declare it can be
    // shared across threads safely.
    unsafe impl<T: Sync + Send> Send for Arc<T> {}
    unsafe impl<T: Sync + Send> Sync for Arc<T> {}

    impl<T> Arc<T> {
        /// Creates a new `Arc` instance.
        ///
        /// # Arguments
        ///
        /// * `data` - The data to be shared.
        pub fn new(data: T) -> Self {
            // Create the inner data structure on the heap.
            // We also inhibit the compiler from automatically calling its
            // destructor using `ManuallyDrop`.
            let mut inner = ManuallyDrop::new(Box::new(ArcInner {
                count: AtomicUsize::new(1),
                data,
            }));

            // We store a reference to the data as a raw pointer
            // (which is `!Sync` and `!Send` by itself).
            let ptr: *mut ArcInner<T> = std::ptr::from_mut(&mut **inner);
            // A reference is just a pointer that is assumed to be aligned, not
            // null, and pointing to memory containing a valid value
            // of `T` in this case. Thus the pointer made from it
            // is as well.
            assert!(!ptr.is_null());
            assert!(ptr.is_aligned());
            println!("Created `Arc` with reference count 1.");
            Arc {
                ptr,
                phantom: PhantomData,
            }
        }

        /// Returns a reference to the inner data.
        ///
        /// SAFETY: this function dereferences a raw pointer.
        /// This said, `new()` guarantees that the inner pointer is valid.
        fn inner(&self) -> &ArcInner<T> {
            assert!(!self.ptr.is_null());
            assert!(self.ptr.is_aligned());

            unsafe { self.ptr.as_ref().unwrap() }
        }

        /// Creates a new `Arc` from a raw pointer to an `ArcInner`.
        ///
        /// SAFETY: This function is unsafe because it takes a raw pointer as
        /// input. The caller must ensure that the pointer is valid and
        /// points to a properly initialized `ArcInner` instance.
        unsafe fn from_inner(ptr: *mut ArcInner<T>) -> Self {
            assert!(!ptr.is_null());
            assert!(ptr.is_aligned());

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
            // Increment the reference count.
            // `count` is atomic, so this is thread-safe.
            let previous_count = self.inner().count.fetch_add(1, Relaxed);
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
            // Dereference the pointer and create a reference to the data.
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
            let previous_count = self.inner().count.fetch_sub(1, Release);
            if previous_count != 1 {
                println!(
                    "Decreased reference count; previous count: {previous_count}"
                );
                return;
            }
            // When the reference count is zero, we drop the underlying data.
            unsafe { std::ptr::drop_in_place(&mut (*self.ptr).data) };
            // You could also use `ManuallyDrop::drop(&mut (*self.ptr))`;

            println!("`Arc` data fully dropped!");
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
                println!("`my_clone` in thread {i} will drop next.")
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    } // At this point, all clones have been dropped and only `my_arc` remains.

    println!("`my_arc` will drop next.");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// LATER miri
