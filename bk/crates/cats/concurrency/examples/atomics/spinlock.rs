#![allow(dead_code)]
// ANCHOR: example
use std::cell::UnsafeCell;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Drop;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread;

/// A simple spinlock implementation.
pub struct Spinlock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>, // Allows mutation through `&self` via unsafe code
}

impl<T> Spinlock<T> {
    /// Creates a new spinlock wrapping the given data.
    pub const fn new(data: T) -> Self {
        Spinlock {
            locked: AtomicBool::new(false), // Start unlocked.
            data: UnsafeCell::new(data),
        }
    }

    /// Acquires the lock, spinning until it is available.
    ///
    /// Returns a guard that allows access to the data and releases the
    /// lock when dropped.
    #[inline]
    pub fn lock(&self) -> SpinlockGuard<T> {
        // Spin until we successfully acquire the lock.
        // `compare_exchange_weak` is often preferred in loops, as it can be
        // more performant on some platforms, even if it spuriously fails.
        while self
            .locked
            .compare_exchange_weak(
                false,             // Current expected value (unlocked).
                true,              // New value if successful (locked).
                Ordering::Acquire, // Memory ordering on success.
                Ordering::Relaxed, // Memory ordering on failure.
            )
            .is_err()
        {
            // Lock is currently held, hint to the CPU that we are spinning.
            // This can improve performance on hyper-threaded CPUs.
            std::hint::spin_loop();
        }
        // We successfully acquired the lock, return the guard
        SpinlockGuard { lock: self }
    }

    // Internal unlock method used by the guard.
    #[inline]
    fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

// Allow the spinlock to be shared across threads if the data is `Send`.
// The `UnsafeCell` requires us to manually declare this.
unsafe impl<T: Send> Sync for Spinlock<T> {}
unsafe impl<T: Send> Send for Spinlock<T> {}

// --- SpinlockGuard Implementation ---

/// An RAII guard that unlocks the spinlock when dropped.
/// If an instance of this struct is created but not used (e.g., assigned to a
/// variable), the lock is immediately released, which is likely unintended.
/// The `must_use` attribute helps prevent this mistake.
#[must_use = "if unused, the spinlock will immediately unlock"]
pub struct SpinlockGuard<'a, T> {
    lock: &'a Spinlock<T>,
}

impl<T> Drop for SpinlockGuard<'_, T> {
    /// Releases the lock when the guard goes out of scope.
    #[inline]
    fn drop(&mut self) {
        self.lock.unlock();
    }
}

impl<T> Deref for SpinlockGuard<'_, T> {
    type Target = T;

    /// Allows immutable access to the protected data.
    #[inline]
    fn deref(&self) -> &Self::Target {
        // Safety: We know the lock is held, because the guard exists.
        // `UnsafeCell::get()` returns a raw pointer. Dereferencing it is
        // unsafe, but justified here by the lock mechanism.
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> DerefMut for SpinlockGuard<'_, T> {
    /// Allows mutable access to the protected data.
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: We know the lock is held, because the guard exists.
        unsafe { &mut *self.lock.data.get() }
    }
}

// --- Example Usage ---
use std::sync::Arc;
use std::time::Duration;

fn main() {
    // Wrap shared data in `Spinlock` and `Arc`.
    let counter = Arc::new(Spinlock::new(0));
    let mut handles = vec![];

    println!("Spawning 5 threads to increment using spinlock...");

    for i in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Acquire the lock - this call spins until successful.
            let mut num = counter_clone.lock();
            // Lock acquired, `num` is the `SpinlockGuard`.

            let current_val = *num;
            println!(
                "Thread {i} acquired lock, value = {current_val}. Incrementing..."
            );

            // Access and modify the data through the guard.
            *num += 1;

            // Simulate some *very* short work while holding the lock.
            // If this work were longer, a `Mutex` would be better.
            thread::sleep(Duration::from_micros(5));

            println!("Thread {i} releasing lock, value = {}", *num);
            // `num` (the guard) goes out of scope here, automatically calling
            // `drop()`, which releases the lock.
        });
        handles.push(handle);
    }

    // Wait for all threads to complete.
    for handle in handles {
        handle.join().expect("Thread panicked.");
    }

    // Access the final value in the main thread.
    let final_value = *counter.lock(); // Lock, dereference, unlock.
    println!("\nFinal counter value: {final_value}");

    assert_eq!(final_value, 5);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
