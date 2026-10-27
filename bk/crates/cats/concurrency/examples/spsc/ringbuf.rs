#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! The `ringbuf` crate as an alternative to `rtrb`: a lock-free SPSC ring
//! buffer with a trait-based API.
//!
//! Real-time threads often also need a higher scheduling priority; see the
//! `thread-priority` crate (`ThreadPriority::Max.set_for_current()`), which is
//! platform-dependent and may require privileges.

use std::thread;

use ringbuf::HeapRb;
use ringbuf::traits::Consumer;
use ringbuf::traits::Producer;
use ringbuf::traits::Split;

/// The number of samples the ring buffer can hold.
const CAPACITY: usize = 32;
/// The total number of samples sent through the buffer.
const TOTAL: usize = 500;

/// Sends `TOTAL` samples through a heap-allocated ring buffer.
fn main() {
    let rb = HeapRb::<f32>::new(CAPACITY);
    let (mut producer, mut consumer) = rb.split();

    let producer_thread = thread::spawn(move || {
        let mut next = 0_usize;
        while next < TOTAL {
            // `try_push` hands the value back if the buffer is full.
            match producer.try_push(next as f32) {
                Ok(()) => next += 1,
                Err(_) => thread::yield_now(),
            }
        }
    });

    let mut received = Vec::with_capacity(TOTAL);
    while received.len() < TOTAL {
        // `try_pop` returns `None` if the buffer is empty.
        match consumer.try_pop() {
            Some(sample) => received.push(sample),
            None => thread::yield_now(),
        }
    }

    match producer_thread.join() {
        Ok(()) => println!("Producer thread finished."),
        Err(e) => eprintln!("Producer thread panicked: {e:?}"),
    }

    assert_eq!(received.len(), TOTAL);
    assert!(
        received
            .iter()
            .enumerate()
            .all(|(i, sample)| *sample == i as f32)
    );
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the ringbuf example through its entry point.
    #[test]
    fn test_ringbuf_example_entry_point() {
        main();
    }
}
