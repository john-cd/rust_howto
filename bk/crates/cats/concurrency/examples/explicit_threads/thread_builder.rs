// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A named thread created with `std::thread::Builder`.
//!
//! The thread builder sets a name and stack size before starting a thread. A
//! name shows up in debuggers, panic messages and profilers, which helps a lot
//! when diagnosing a multi-threaded program.

use std::thread;

use anyhow::anyhow;

/// Creates a named worker with a custom stack size and returns its name.
fn worker_name() -> anyhow::Result<String> {
    let handle = thread::Builder::new()
        .name(String::from("metadata-worker"))
        .stack_size(2 * 1024 * 1024)
        .spawn(|| {
            thread::current()
                .name()
                .map(str::to_owned)
                .ok_or_else(|| anyhow!("the worker thread has no name"))
        })?;

    handle
        .join()
        .map_err(|_| anyhow!("the named worker thread panicked"))?
}

/// Creates a named thread and checks the available parallelism hint.
fn main() -> anyhow::Result<()> {
    let worker = worker_name()?;
    let parallelism = thread::available_parallelism()?.get();

    assert_eq!(worker, "metadata-worker");
    assert!(parallelism > 0);
    println!("Worker {worker}; available parallelism: {parallelism}");
    Ok(())
}

// ANCHOR_END: example

/// Runs the thread-builder example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the builder applies the name and returns a usable hint.
    #[test]
    fn builder_names_worker_and_reports_parallelism() -> anyhow::Result<()> {
        assert_eq!(worker_name()?, "metadata-worker");
        assert!(thread::available_parallelism()?.get() > 0);
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
