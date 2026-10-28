// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! `static mut` Example.
//!
//! `static mut` use is generally discouraged due to the potential for data
//! races and undefined behavior. The following demonstrates alternatives.
//! See <https://doc.rust-lang.org/nightly/edition-guide/rust-2024/static-mut-references.html>

use std::sync::Mutex;
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering;
use std::thread;

/// Modifies a mutable static, which requires `unsafe` and is discouraged.
fn dont_do_this() {
    // Declare a mutable static variable:
    /// Mutable global integer used to demonstrate `static mut` access.
    static mut VAL: i32 = 0;

    // An unsafe block is required when either reading or writing a mutable
    // static variable. Care should be taken to ensure that modifications to
    // a mutable static are safe with respect to other threads running in the
    // same process.
    unsafe {
        VAL += 1; // Single-threaded, thus OK.
        let v = VAL;
        assert_eq!(v, 1);
    }
    // Creating references to a mutable static generates compiler errors,
    // really limiting what you can do with a `static mut`.

    // BEWARE: In addition, statics can be initialized from mutable statics,
    // but they read the _initial_ value of that static.
    /// Value copied from `VAL` during static initialization.
    static READ_FROM_VAL: i32 = unsafe { VAL };
    assert_eq!(READ_FROM_VAL, 0);
}

/// Uses a static atomic instead of a mutable static.
fn do_instead() {
    // Use an _immutable_ static containing an Atomic.
    // Note that Atomic variables are safe to share between threads (they
    // implement `Sync`).
    /// Atomic counter that supports concurrent updates without a mutex.
    static COUNTER: AtomicU16 = AtomicU16::new(0);

    println!("Initial COUNTER value: {}", COUNTER.load(Ordering::Relaxed));
    // Be sure to analyze your use case to determine the correct `Ordering` to
    // use.

    let old_count = COUNTER.fetch_add(1, Ordering::Relaxed);
    println!(
        "COUNTER after incrementing on the main thread: {}",
        old_count + 1
    );
    assert_eq!(COUNTER.load(Ordering::Relaxed), 1);

    // Spin up two new threads to further increment the static:
    let handle1 = thread::spawn(|| {
        for _ in 0..10_000 {
            COUNTER.fetch_add(1, Ordering::Relaxed);
        }
    });

    let handle2 = thread::spawn(|| {
        for _ in 0..10_000 {
            COUNTER.fetch_add(1, Ordering::Relaxed);
        }
    });

    handle1.join().expect("worker thread should not panic");
    handle2.join().expect("worker thread should not panic");

    let final_count = COUNTER.load(Ordering::Relaxed);
    println!("Final COUNTER value: {final_count}");
    assert_eq!(final_count, 20_001);
}

/// Uses a static `Mutex` to share mutable state.
fn or_do_that() {
    // The preferred way to handle mutable global state is to use
    // synchronization primitives with interior mutability
    // within an immutable `static` variable.

    // Use `Mutex` or `RwLock`, potentially wrapped in an `Arc`,
    // for safe concurrent access:
    /// Counter protected by a mutex for safe shared mutation.
    static SAFE_COUNTER: Mutex<u32> = Mutex::new(0);

    println!(
        "Initial SAFE_COUNTER value: {}",
        SAFE_COUNTER.lock().expect("mutex should not be poisoned")
    );

    let safe_handle1 = thread::spawn(|| {
        for _ in 0..10_000 {
            let mut num =
                SAFE_COUNTER.lock().expect("mutex should not be poisoned");
            *num += 1;
        }
    });

    let safe_handle2 = thread::spawn(|| {
        for _ in 0..10_000 {
            let mut num =
                SAFE_COUNTER.lock().expect("mutex should not be poisoned");
            *num += 1;
        }
    });

    safe_handle1.join().expect("worker thread should not panic");
    safe_handle2.join().expect("worker thread should not panic");

    let final_count =
        *SAFE_COUNTER.lock().expect("mutex should not be poisoned");
    println!("Final SAFE_COUNTER value: {final_count}");
    assert_eq!(final_count, 20_000);
}

/// Runs the three approaches in turn.
fn main() {
    dont_do_this();
    do_instead();
    or_do_that();
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the static mut example through its entry point.
    #[test]
    fn test_static_mut_example_entry_point() {
        main();
    }
}
