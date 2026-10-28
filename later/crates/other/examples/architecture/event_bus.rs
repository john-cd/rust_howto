// TODO Reviewed ??-??-2026.

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

/// A minimal in-process bus with bounded subscriber queues.
#[derive(Default)]
struct EventBus {
    subscribers: Vec<SyncSender<Event>>,
}

impl EventBus {
    /// Adds a subscriber and returns its event receiver.
    fn subscribe(&mut self) -> Receiver<Event> {
        let (tx, rx) = mpsc::sync_channel(16);
        self.subscribers.push(tx);
        rx
    }

    /// Sends an event to connected subscribers and returns disconnections.
    ///
    /// A full subscriber queue blocks until it has space or disconnects.
    fn publish(&mut self, event: &Event) -> usize {
        let before = self.subscribers.len();
        self.subscribers
            .retain(|subscriber| subscriber.send(event.clone()).is_ok());
        before - self.subscribers.len()
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

    assert_eq!(bus.publish(&Event::OrderPlaced { id: 1 }), 0);
    assert_eq!(bus.publish(&Event::OrderShipped { id: 1 }), 0);
    drop(bus); // Closes the channels, which ends the consumers.

    for consumer in consumers {
        let events = consumer.join().expect("consumer panicked");
        assert_eq!(
            events,
            vec![Event::OrderPlaced { id: 1 }, Event::OrderShipped { id: 1 }]
        );
        println!("{events:?}");
    }
}
// ANCHOR_END: example

/// Runs the event-bus example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the event bus example through its entry point.
    #[test]
    fn test_event_bus_example_entry_point() {
        main();
    }

    /// Verifies delivery to connected subscribers and removal of closed ones.
    #[test]
    fn reports_and_removes_disconnected_subscribers() {
        let mut bus = EventBus::default();
        let receiver = bus.subscribe();
        let disconnected = bus.subscribe();
        drop(disconnected);

        let event = Event::OrderPlaced { id: 42 };
        assert_eq!(bus.publish(&event), 1);
        assert_eq!(
            receiver
                .recv()
                .expect("connected subscriber receives the event"),
            event
        );
        assert_eq!(bus.publish(&event), 0);
    }
}
