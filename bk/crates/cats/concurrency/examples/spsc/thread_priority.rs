// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Raise the scheduling priority of a real-time consumer thread with
//! `thread-priority`.
//!
//! A thread that feeds an audio device or a game loop must be scheduled
//! promptly. `ThreadBuilder` spawns a thread and applies a priority before the
//! closure runs. Raising a priority often needs privileges (for example
//! `CAP_SYS_NICE` on Linux), so the closure receives the outcome and the
//! program carries on at the normal priority when the request is refused.

use anyhow::Result;
use rtrb::PopError;
use rtrb::PushError;
use rtrb::RingBuffer;
use thread_priority::ThreadBuilder;
use thread_priority::ThreadPriority;

/// The number of samples the ring buffer can hold.
const CAPACITY: usize = 64;
/// The total number of samples sent through the buffer.
const TOTAL: usize = 500;

/// What the consumer thread reports back.
#[derive(Debug)]
struct Report {
    /// Whether the operating system accepted the requested priority.
    priority_granted: bool,
    /// The samples received, in order.
    samples: Vec<f32>,
}

/// Spawns a consumer thread that requests `priority`, then drains `TOTAL`
/// samples from a ring buffer filled by the calling thread.
///
/// # Errors
///
/// Returns an error if a thread cannot be spawned or panics.
fn consume_with_priority(priority: ThreadPriority) -> Result<Report> {
    let (mut producer, mut consumer) = RingBuffer::<f32>::new(CAPACITY);

    let consumer_thread = ThreadBuilder::default()
        .name("realtime-consumer")
        .priority(priority)
        .spawn(move |priority_result| {
            // A refused priority is not fatal: report it and continue.
            let priority_granted = priority_result.is_ok();
            let mut samples = Vec::with_capacity(TOTAL);
            while samples.len() < TOTAL {
                match consumer.pop() {
                    Ok(sample) => samples.push(sample),
                    Err(PopError::Empty) => {
                        if consumer.is_abandoned() && consumer.is_empty() {
                            break;
                        }
                        std::thread::yield_now();
                    }
                }
            }
            Report {
                priority_granted,
                samples,
            }
        })?;

    let mut next = 0_usize;
    while next < TOTAL {
        match producer.push(next as f32) {
            Ok(()) => next += 1,
            Err(PushError::Full(_)) => std::thread::yield_now(),
        }
    }

    consumer_thread
        .join()
        .map_err(|_| anyhow::anyhow!("consumer thread panicked"))
}

/// Requests the maximum priority and prints whether it was granted.
///
/// # Errors
///
/// Returns an error if the consumer thread cannot be spawned or panics.
fn main() -> Result<()> {
    let report = consume_with_priority(ThreadPriority::Max)?;
    if report.priority_granted {
        println!("Running with the maximum priority.");
    } else {
        println!("Priority change refused; running at the normal priority.");
    }
    assert_eq!(report.samples.len(), TOTAL);
    assert!(
        report
            .samples
            .iter()
            .enumerate()
            .all(|(i, sample)| *sample == i as f32)
    );
    println!("Received {} samples in order.", report.samples.len());
    Ok(())
}
// ANCHOR_END: example

/// Runs the thread priority example.
///
/// # Errors
///
/// Returns an error if the consumer thread cannot be spawned or panics.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point, whatever the privileges.
    #[test]
    fn test_thread_priority_example_entry_point() -> Result<()> {
        main()
    }

    /// Delivers every sample in order, whether or not the priority is granted.
    #[test]
    fn consumer_receives_all_samples_in_order() -> Result<()> {
        let report = consume_with_priority(ThreadPriority::Min)?;
        assert_eq!(report.samples.len(), TOTAL);
        assert!(
            report
                .samples
                .iter()
                .enumerate()
                .all(|(i, sample)| *sample == i as f32)
        );
        Ok(())
    }
}
