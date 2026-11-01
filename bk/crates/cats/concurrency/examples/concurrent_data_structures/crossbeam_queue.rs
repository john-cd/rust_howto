// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates the basic usage of `ArrayQueue`.
//!
//! A lock-free queue lets many threads push and pop without taking a mutex, so
//! a stalled thread never blocks the others. Work queues, schedulers and
//! logging pipelines use them.

use crossbeam_queue::ArrayQueue;

/// Demonstrates the basic usage of `ArrayQueue`.
fn main() -> anyhow::Result<()> {
    // Create a new `ArrayQueue` with a capacity of 2.
    let q = ArrayQueue::new(2);
    // Push 'a' onto the queue. This should succeed.
    assert_eq!(q.push('a'), Ok(()));
    // Push 'b' onto the queue. This should also succeed.
    assert_eq!(q.push('b'), Ok(()));
    assert_eq!(q.push('c'), Err('c'));
    assert_eq!(q.pop(), Some('a'));
    assert_eq!(q.pop(), Some('b'));
    assert_eq!(q.pop(), None);
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the bounded queue accepts values up to capacity in FIFO
    /// order.
    /// Runs the crossbeam queue example through its entry point.
    #[test]
    fn test_crossbeam_queue_example_entry_point() {
        main().expect("queue recipe should pass its assertions");
    }
}
