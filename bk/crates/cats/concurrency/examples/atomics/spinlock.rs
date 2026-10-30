// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A spinlock built on an atomic flag, with a guard that unlocks it on drop.

use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Drop;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread;

/// A simple spinlock implementation.
pub struct Spinlock<T> {
    /// Whether the lock is currently held.
    locked: AtomicBool,
    /// The protected data.
    data: UnsafeCell<T>, // Allows mutation through `&self` via unsafe code.
}

impl<T> Spinlock<T> {
    /// Creates a new spinlock wrapping the given data.
    pub const fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false), // Start unlocked.
            data: UnsafeCell::new(data),
        }
    }

    /// Acquires the lock, spinning until it is available.
    ///
    /// Returns a guard that allows access to the data and releases the
    /// lock when dropped.
    #[inline]
    pub fn lock(&self) -> SpinlockGuard<'_, T> {
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
        // We successfully acquired the lock, return the guard:
        SpinlockGuard {
            lock: self,
            exclusive: PhantomData,
        }
    }

    // Internal unlock method used by the guard.
    /// Releases the lock.
    #[inline]
    fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

// SAFETY: `lock` serializes access to the `UnsafeCell`, and its guard keeps
// the flag held until all references to the protected value are dropped. The
// `Send` bound allows the protected value to move between threads.
// SAFETY: `lock` grants exclusive access through its guard, and `T: Send`
// allows the protected value to move between threads. The guard's exclusive
// marker prevents sharing a guard unless `T: Sync`.
unsafe impl<T: Send> Sync for Spinlock<T> {}

// --- SpinlockGuard Implementation ---

/// An RAII guard that unlocks the spinlock when dropped.
/// If an instance of this struct is created but not used (e.g., assigned to a
/// variable), the lock is immediately released, which is likely unintended.
/// The `must_use` attribute helps prevent this mistake.
#[must_use = "if unused, the spinlock will immediately unlock"]
pub struct SpinlockGuard<'a, T> {
    /// The spinlock that this guard holds.
    lock: &'a Spinlock<T>,
    /// Keeps the guard's auto-traits aligned with exclusive access to `T`.
    exclusive: PhantomData<&'a mut T>,
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
/// Increments a shared counter from several threads under the spinlock.
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
            // Keep the critical section short: only update the value here.
            *num += 1;
            let value = *num;
            // Release the lock before doing slower output.
            drop(num);
            println!("Thread {i} incremented the counter to {value}.");
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

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::panic::AssertUnwindSafe;
    use std::panic::catch_unwind;

    use static_assertions::assert_not_impl_any;

    use super::*;

    assert_not_impl_any!(SpinlockGuard<'static, Cell<u8>>: Sync);

    /// Requires a type to implement `Send` at compile time.
    fn assert_send<T: Send>() {}

    /// Requires a type to implement `Sync` at compile time.
    fn assert_sync<T: Sync>() {}

    /// Checks that the guard is shareable only when its payload is `Sync`.
    #[test]
    fn guard_auto_traits_follow_payload_bounds() {
        assert_send::<SpinlockGuard<'static, Cell<u8>>>();
        assert_send::<SpinlockGuard<'static, u8>>();
        assert_sync::<SpinlockGuard<'static, u8>>();
    }

    /// Verifies unwinding drops the guard and releases the lock.
    #[test]
    fn guard_unlocks_during_unwind() {
        let lock = Spinlock::new(0);
        let result = catch_unwind(AssertUnwindSafe(|| {
            let _guard = lock.lock();
            panic!("test unwind while holding the guard");
        }));

        assert!(result.is_err());
        assert_eq!(*lock.lock(), 0);
    }

    /// Runs the spinlock example through its entry point.
    #[test]
    fn test_spinlock_example_entry_point() {
        main();
    }
}
