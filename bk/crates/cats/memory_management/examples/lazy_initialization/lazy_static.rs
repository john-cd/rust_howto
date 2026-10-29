// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! `lazy_static` defines statically initialized values that are
//! computed lazily at runtime. It can be particularly useful for initializing
//! data that is expensive to compute or that needs to be shared across multiple
//! threads. Since Rust 1.80, `std::sync::LazyLock` is the standard-library
//! replacement for this pattern in new code.

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::MutexGuard;

use anyhow::anyhow;
use lazy_static::lazy_static;

// The `lazy_static` macro generates code to ensure that the static variable is
// initialized only once, the first time it's accessed.
// - Any type within the macro needs to fulfill the Sync trait.
// - If a type has a destructor, it will not run when the process exits.
// - The `static ref` keywords are passed to the macro, they are not part of the
//   language per se.
lazy_static! {
    // `CONFIG` is a lazily-initialized HashMap that holds configuration data.
/// Configuration map initialized once and shared by all callers.
    static ref CONFIG: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("host".to_owned(), "localhost".to_owned());
        m.insert("port".to_owned(), "8080".to_owned());
        m
    };
    // You can call a method or a function:
/// Number of entries in the lazily initialized configuration map.
    static ref COUNT: usize = CONFIG.len();

    // `COUNTER` is a lazily-initialized Mutex-protected integer counter.
    // The `Mutex` ensures that the COUNTER can be safely incremented from multiple threads, if needed.
/// Mutex-protected counter initialized on first use.
    static ref COUNTER: Mutex<i32> = Mutex::new(0);
}

/// Reads values from a lazily initialized global map.
fn main() -> anyhow::Result<()> {
    // Accessing the CONFIG.
    println!(
        "Host: {}",
        CONFIG.get("host").ok_or(anyhow!("No host key"))?
    );
    println!(
        "Port: {}",
        CONFIG.get("port").ok_or(anyhow!("No port key"))?
    );

    // The macro generates a unique type that implements `Deref<TYPE>` and
    // stores it in a static with name `NAME`. Here, we use `*` to dereference
    // and retrieve the inner value.
    println!("Count: {}", *COUNT);

    // Working with the COUNTER:
    // The (mutable) shared state within the Mutex can only be accessed once the
    // lock is held. Our non-atomic increment is safe because we're the only
    // thread which can access the shared state when the lock is held.
    {
        let mut counter: MutexGuard<'_, i32> = COUNTER.lock().expect(
            "counter mutex is not poisoned before this example locks it",
        );
        *counter += 1;
        println!("Counter: {}", *counter);
    } // The lock is unlocked here.

    // And again, on a different thread:
    std::thread::spawn(move || {
        let lock_result = COUNTER.try_lock();
        // If the lock cannot be acquired at this time (or is poisoned),
        // `Err` is returned.
        if let Ok(mut counter) = lock_result {
            *counter += 1;
            println!("Counter: {}", *counter);
        } else {
            println!("try_lock failed");
        }
    })
    .join()
    .map_err(|_| anyhow!("counter worker thread panicked"))?;

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies lazy values and synchronized counter access work together.
    /// Runs the lazy static example through its entry point.
    #[test]
    fn test_lazy_static_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// The configuration and derived count are consistent, and the counter
    /// can be locked.
    #[test]
    fn config_and_counter_are_consistent() -> anyhow::Result<()> {
        assert_eq!(CONFIG.get("host").map(String::as_str), Some("localhost"));
        assert_eq!(CONFIG.get("port").map(String::as_str), Some("8080"));
        assert_eq!(CONFIG.get("missing"), None);
        assert_eq!(*COUNT, CONFIG.len());
        let counter = COUNTER
            .lock()
            .map_err(|_| anyhow!("counter mutex poisoned"))?;
        assert!(*counter >= 0);
        Ok(())
    }
}
