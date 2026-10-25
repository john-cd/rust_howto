#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Event-driven architecture: producers publish events to subscribers.

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use std::thread;

/// Events published on the bus.
#[derive(Debug, Clone, PartialEq)]
enum Event {
    OrderPlaced { id: u32 },
    OrderShipped { id: u32 },
}

/// A minimal bus that delivers each event to every subscriber.
#[derive(Default)]
struct EventBus {
    subscribers: Vec<SyncSender<Event>>,
}

impl EventBus {
    fn subscribe(&mut self) -> Receiver<Event> {
        let (tx, rx) = mpsc::sync_channel(16);
        self.subscribers.push(tx);
        rx
    }

    fn publish(&self, event: &Event) {
        for subscriber in &self.subscribers {
            // A subscriber that has gone away is ignored.
            let _ = subscriber.send(event.clone());
        }
    }
}

/// Publishes events and collects them in two independent consumers.
fn main() {
    let mut bus = EventBus::default();
    let billing = bus.subscribe();
    let audit = bus.subscribe();

    let consumers: Vec<_> = [billing, audit]
        .into_iter()
        .map(|rx| thread::spawn(move || rx.iter().collect::<Vec<Event>>()))
        .collect();

    bus.publish(&Event::OrderPlaced { id: 1 });
    bus.publish(&Event::OrderShipped { id: 1 });
    drop(bus); // Closes the channels, which ends the consumers.

    for consumer in consumers {
        let events = consumer.join().expect("consumer panicked");
        assert_eq!(events.len(), 2);
        println!("{events:?}");
    }
}
// ANCHOR_END: example

/// Runs the example.
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
}
