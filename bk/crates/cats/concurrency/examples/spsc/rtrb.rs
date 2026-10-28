// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A wait-free single-producer, single-consumer (SPSC) ring buffer from the
//! `rtrb` crate.
//!
//! The buffer never allocates after creation and never blocks, which is what a
//! real-time thread (audio callback, game loop) needs: `push` and `pop` return
//! an error instead of waiting when the buffer is full or empty.

use std::thread;

use anyhow::Result;
use rtrb::Consumer;
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

/// Drains at most one block without waiting, filling an underrun with silence.
///
/// This function neither allocates nor yields; it handles the sample transfer
/// portion of a real-time callback.
fn drain_block(
    consumer: &mut Consumer<f32>,
    block: &mut [f32; BLOCK_SIZE],
) -> usize {
    block.fill(0.0);
    let mut filled = 0;
    for slot in block {
        match consumer.pop() {
            Ok(sample) => {
                *slot = sample;
                filled += 1;
            }
            Err(PopError::Empty) => break,
        }
    }
    filled
}

/// Sends `TOTAL` samples and drives the block consumer from an ordinary thread.
///
/// # Errors
///
/// Returns an error if the producer thread panics.
fn main() -> Result<()> {
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

    // This is a test driver, not a real-time callback: it may yield while
    // waiting for the producer. A callback calls `drain_block` only once.
    let mut received = Vec::with_capacity(TOTAL);
    let mut block = [0.0_f32; BLOCK_SIZE];
    while received.len() < TOTAL {
        let filled = drain_block(&mut consumer, &mut block);
        received.extend_from_slice(&block[..filled]);

        // Stop if the producer is gone and nothing is left.
        if filled == 0 {
            if consumer.is_abandoned() && consumer.is_empty() {
                break;
            }
            thread::yield_now();
        }
    }

    producer_thread
        .join()
        .map_err(|_| anyhow::anyhow!("sample producer thread panicked"))?;

    // All samples arrived, in order.
    assert_eq!(received.len(), TOTAL);
    assert!(
        received
            .iter()
            .enumerate()
            .all(|(i, sample)| *sample == i as f32)
    );
    println!("Received {} samples in order.", received.len());
    Ok(())
}
// ANCHOR_END: example

/// Runs the SPSC transfer example and reports producer thread failures.
///
/// # Errors
///
/// Returns an error if the producer thread panics.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the rtrb example through its entry point.
    #[test]
    fn test_rtrb_example_entry_point() -> Result<()> {
        main()
    }

    /// Returns full/empty errors without blocking or losing buffered values.
    #[test]
    fn test_full_and_empty() {
        full_and_empty();
    }

    /// Drains available samples once and fills the rest of the block with
    /// silence.
    #[test]
    fn drains_one_block_and_silences_underruns() {
        let (mut producer, mut consumer) = RingBuffer::new(2);
        assert!(producer.push(1.0).is_ok());
        assert!(producer.push(2.0).is_ok());
        let mut block = [9.0; BLOCK_SIZE];

        assert_eq!(drain_block(&mut consumer, &mut block), 2);
        assert_eq!(&block[..2], &[1.0, 2.0]);
        assert!(block[2..].iter().all(|sample| *sample == 0.0));
        assert_eq!(drain_block(&mut consumer, &mut block), 0);
        assert_eq!(block, [0.0; BLOCK_SIZE]);
    }
}
