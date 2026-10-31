// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Scoped threads borrow a local string and return values through their
//! handles.
//!
//! A scoped thread may borrow local variables, because the scope guarantees
//! that all its threads finish before it returns. Plain `thread::spawn` cannot
//! do that: its closure must own its data.

use std::thread;

use anyhow::anyhow;

/// Borrows local text from scoped threads and returns their joined results.
fn scoped_results() -> anyhow::Result<(usize, usize)> {
    let text = String::from("Rust threads");

    thread::scope(|scope| -> anyhow::Result<(usize, usize)> {
        let byte_count = scope.spawn(|| text.len());
        let word_count = scope.spawn(|| text.split_whitespace().count());

        let byte_count = byte_count
            .join()
            .map_err(|_| anyhow!("the byte-count worker thread panicked"))?;
        let word_count = word_count
            .join()
            .map_err(|_| anyhow!("the word-count worker thread panicked"))?;

        Ok((byte_count, word_count))
    })
}

/// Uses scoped workers to borrow local data and checks their results.
fn main() -> anyhow::Result<()> {
    let results = scoped_results()?;
    assert_eq!(results, (12, 2));
    println!("Scoped results: {results:?}");
    Ok(())
}

// ANCHOR_END: example

/// Runs the scoped-thread example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that scoped workers borrow local text and return their values.
    #[test]
    fn scoped_threads_join_borrowing_workers() -> anyhow::Result<()> {
        assert_eq!(scoped_results()?, (12, 2));
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
