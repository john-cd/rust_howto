//! Encodes messages with `postcard` into a bounded `heapless` queue.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
use heapless::Deque;
use heapless::Vec;
use serde::Deserialize;
use serde::Serialize;

/// A message with a stable wire format.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Reading {
    sensor: u8,
    millidegrees: i32,
}

/// The queue holds at most four encoded messages of up to eight bytes each.
type Queue = Deque<Vec<u8, 8>, 4>;

/// Encodes `reading` and queues it, or returns it when the queue is full.
///
/// A bounded queue forces the application to choose a policy; this one
/// rejects the new message instead of overwriting old data.
fn enqueue(queue: &mut Queue, reading: &Reading) -> anyhow::Result<bool> {
    let mut buffer = [0_u8; 8];
    let encoded = postcard::to_slice(reading, &mut buffer)?;
    let message = Vec::from_slice(encoded)
        .map_err(|_| anyhow::anyhow!("message exceeds the buffer"))?;
    Ok(queue.push_back(message).is_ok())
}

/// Decodes the oldest queued message.
fn dequeue(queue: &mut Queue) -> anyhow::Result<Option<Reading>> {
    queue
        .pop_front()
        .map(|bytes| Ok(postcard::from_bytes(&bytes)?))
        .transpose()
}

/// Queues six readings into a four-slot queue and drains it.
fn main() -> anyhow::Result<()> {
    let mut queue = Queue::new();
    for sensor in 0..6 {
        let reading = Reading {
            sensor,
            millidegrees: 21_500 + i32::from(sensor),
        };
        let accepted = enqueue(&mut queue, &reading)?;
        println!("sensor {sensor}: accepted = {accepted}");
    }
    while let Some(reading) = dequeue(&mut queue)? {
        println!("{reading:?}");
    }
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    fn test_main_entry_point() -> anyhow::Result<()> {
        main()
    }

    /// Verifies that a full queue rejects new messages and keeps the old ones.
    #[test]
    fn a_full_queue_rejects_new_messages() -> anyhow::Result<()> {
        let mut queue = Queue::new();
        for sensor in 0..4 {
            let reading = Reading {
                sensor,
                millidegrees: 0,
            };
            assert!(enqueue(&mut queue, &reading)?);
        }
        let overflow = Reading {
            sensor: 9,
            millidegrees: 0,
        };
        assert!(!enqueue(&mut queue, &overflow)?);
        assert_eq!(dequeue(&mut queue)?.map(|r| r.sensor), Some(0));
        Ok(())
    }

    /// Verifies that a message survives encoding and decoding.
    #[test]
    fn messages_round_trip() -> anyhow::Result<()> {
        let mut queue = Queue::new();
        let reading = Reading {
            sensor: 3,
            millidegrees: -4_250,
        };
        assert!(enqueue(&mut queue, &reading)?);
        assert_eq!(dequeue(&mut queue)?, Some(reading));
        assert_eq!(dequeue(&mut queue)?, None);
        Ok(())
    }
}
