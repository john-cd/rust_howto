// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates using a `Mutex` to safely share mutable state between threads.
//!
//! A `Mutex` allows one thread at a time to access the data it protects, which
//! prevents data races. `Arc` lets several threads own the mutex. Hold the lock
//! for as short a time as possible, and never wait on something else while
//! holding it.

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use anyhow::anyhow;

/// Increments one shared counter from several threads.
fn count_increments(
    thread_count: usize,
    increments_per_thread: usize,
) -> anyhow::Result<usize> {
    let counter = Arc::new(Mutex::new(0));
    let handles = (0..thread_count)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || -> anyhow::Result<()> {
                for _ in 0..increments_per_thread {
                    *counter.lock().map_err(|_| {
                        anyhow!("counter mutex was poisoned")
                    })? += 1;
                }
                Ok(())
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle
            .join()
            .map_err(|_| anyhow!("counter worker thread panicked"))??;
    }

    let total = *counter
        .lock()
        .map_err(|_| anyhow!("counter mutex was poisoned"))?;
    Ok(total)
}

/// Demonstrates using a mutex to serialize concurrent counter updates.
fn main() -> anyhow::Result<()> {
    let total = count_increments(10, 100)?;
    assert_eq!(total, 1_000);
    println!("Final counter: {total}");
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that every worker increment is present after all joins.
    #[test]
    fn mutex_counter_contains_all_worker_increments() -> anyhow::Result<()> {
        assert_eq!(count_increments(10, 100)?, 1_000);
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
