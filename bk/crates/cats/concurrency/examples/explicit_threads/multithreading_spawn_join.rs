// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Spawn threads and collect their results through `JoinHandle::join`.

use std::thread;

use anyhow::anyhow;

/// Spawns two threads and returns their results, reporting worker panics.
fn joined_results() -> anyhow::Result<(u32, u32)> {
    let first = thread::spawn(|| 40 + 2);
    let second = thread::spawn(|| 20 * 2);

    let first_result = first
        .join()
        .map_err(|_| anyhow!("the first worker thread panicked"))?;
    let second_result = second
        .join()
        .map_err(|_| anyhow!("the second worker thread panicked"))?;

    Ok((first_result, second_result))
}

/// Spawns two workers and asserts their returned values.
fn main() -> anyhow::Result<()> {
    let results = joined_results()?;
    assert_eq!(results, (42, 40));
    println!("Joined results: {results:?}");
    Ok(())
}

// ANCHOR_END: example

/// Runs the spawn-and-join example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that both joined workers return their computed values.
    #[test]
    fn spawned_threads_return_joined_values() -> anyhow::Result<()> {
        assert_eq!(joined_results()?, (42, 40));
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
