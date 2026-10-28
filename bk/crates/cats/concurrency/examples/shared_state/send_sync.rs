// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates safe concurrent access to shared data
//! using `Arc` (Atomic Reference Counting) and `Mutex` (Mutual Exclusion).
//!
//! - `Arc` allows multiple threads to have shared ownership of the data.
//! - `Mutex` ensures that only one thread can access the data at a time.
//!
//! Under the covers, `Mutex<T>` is `Send + Sync` if `T` is `Send`.
//! Note that `T: Sync` is not required, as the underlying data is only made
//! available to one thread at a time.

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use anyhow::anyhow;

/// Increments a counter shared by several threads with `Arc` and `Mutex`.
fn increments_with_shared_mutex() -> anyhow::Result<usize> {
    let data = Arc::new(Mutex::new(0));
    let handles = (0..3)
        .map(|_| {
            let data = Arc::clone(&data);
            thread::spawn(move || -> anyhow::Result<()> {
                for _ in 0..50 {
                    *data.lock().map_err(|_| {
                        anyhow!("shared-data mutex was poisoned")
                    })? += 1;
                }
                Ok(())
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle
            .join()
            .map_err(|_| anyhow!("shared-data worker thread panicked"))??;
    }

    let total = *data
        .lock()
        .map_err(|_| anyhow!("shared-data mutex was poisoned"))?;
    Ok(total)
}

/// Demonstrates that `Arc<Mutex<T>>` can share `T: Send` between threads.
fn main() -> anyhow::Result<()> {
    let result = increments_with_shared_mutex()?;
    assert_eq!(result, 150);
    println!("Final shared value: {result}");
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the shared mutex protects all concurrent increments.
    #[test]
    fn shared_mutex_returns_the_total_number_of_increments()
    -> anyhow::Result<()> {
        assert_eq!(increments_with_shared_mutex()?, 150);
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
