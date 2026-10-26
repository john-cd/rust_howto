#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A wait-free single-producer, single-consumer (SPSC) ring buffer from the
//! `rtrb` crate.
//!
//! The buffer never allocates after creation and never blocks, which is what a
//! real-time thread (audio callback, game loop) needs: `push` and `pop` return
//! an error instead of waiting when the buffer is full or empty.

use std::thread;

use rtrb::PopError;
use rtrb::PushError;
use rtrb::RingBuffer;

/// The number of samples the ring buffer can hold.
const CAPACITY: usize = 64;
/// The number of samples the consumer drains at a time.
const BLOCK_SIZE: usize = 16;
/// The total number of samples sent through the buffer.
const TOTAL: usize = 1_000;

/// Shows what happens when the buffer is full or empty.
fn full_and_empty() {
    let (mut producer, mut consumer) = RingBuffer::<f32>::new(2);

    assert_eq!(consumer.pop(), Err(PopError::Empty));

    assert!(producer.push(1.0).is_ok());
    assert!(producer.push(2.0).is_ok());
    // The third `push` does not block: it hands the value back.
    assert_eq!(producer.push(3.0), Err(PushError::Full(3.0)));

    assert_eq!(consumer.pop(), Ok(1.0));
    assert_eq!(consumer.pop(), Ok(2.0));
    assert_eq!(consumer.pop(), Err(PopError::Empty));
}

/// Sends `TOTAL` samples from a producer thread and drains them in fixed-size
/// blocks on the consumer side.
fn main() {
    full_and_empty();

    let (mut producer, mut consumer) = RingBuffer::<f32>::new(CAPACITY);

    // The "non-real-time" side: it may spin while the buffer is full.
    let producer_thread = thread::spawn(move || {
        let mut next = 0_usize;
        while next < TOTAL {
            match producer.push(next as f32) {
                Ok(()) => next += 1,
                Err(PushError::Full(_)) => thread::yield_now(),
            }
        }
    });

    // The "real-time" side: it never waits on a lock and never allocates in
    // the loop, other than the `received` vector we use for the assertions.
    let mut received = Vec::with_capacity(TOTAL);
    let mut block = [0.0_f32; BLOCK_SIZE];
    while received.len() < TOTAL {
        let mut filled = 0;
        while filled < BLOCK_SIZE {
            match consumer.pop() {
                Ok(sample) => {
                    block[filled] = sample;
                    filled += 1;
                }
                Err(PopError::Empty) => break,
            }
        }
        received.extend_from_slice(&block[..filled]);

        // Stop if the producer is gone and nothing is left.
        if filled == 0 {
            if consumer.is_abandoned() && consumer.is_empty() {
                break;
            }
            thread::yield_now();
        }
    }

    match producer_thread.join() {
        Ok(()) => println!("Producer thread finished."),
        Err(e) => eprintln!("Producer thread panicked: {e:?}"),
    }

    // All samples arrived, in order.
    assert_eq!(received.len(), TOTAL);
    assert!(
        received
            .iter()
            .enumerate()
            .all(|(i, sample)| *sample == i as f32)
    );
    println!("Received {} samples in order.", received.len());
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_full_and_empty() {
        full_and_empty();
    }
}
