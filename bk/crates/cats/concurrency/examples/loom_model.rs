// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Model-checks a publication and cancellation protocol with `loom`.
//!
//! One thread publishes a value and then raises a `ready` flag; the other
//! thread may cancel the producer and reads the value only after observing
//! `ready`. `loom::model` runs the closure repeatedly and explores the
//! interleavings (and the visible values of atomics) that the C11 memory model
//! allows, within a preemption bound. The invariant is: a reader that sees
//! `ready` always sees the published value, and `ready` is set exactly when the
//! producer reports that it published.
//!
//! The code under test must use `loom::sync`, `loom::thread` and `loom::cell`
//! instead of their `std` equivalents. Real projects usually switch the types
//! with `#[cfg(loom)]` and run the model with `RUSTFLAGS="--cfg loom"`; here
//! the model uses `loom` types directly to stay self-contained.

use loom::model::Builder;
use loom::sync::Arc;
use loom::sync::atomic::AtomicBool;
use loom::sync::atomic::AtomicU32;
use loom::sync::atomic::Ordering;
use loom::thread;

/// Shared state of the protocol.
struct Slot {
    /// The published payload (0 means "not published yet").
    value: AtomicU32,
    /// Raised by the producer after it stored the payload.
    ready: AtomicBool,
    /// Raised by the consumer to stop the producer.
    cancelled: AtomicBool,
    /// Ordering used by the producer to raise `ready`.
    publish_order: Ordering,
    /// Ordering used by the consumer to read `ready`.
    read_order: Ordering,
}

impl Slot {
    /// Creates an empty slot that raises and reads `ready` with the given
    /// orderings.
    fn new(publish_order: Ordering, read_order: Ordering) -> Self {
        Self {
            value: AtomicU32::new(0),
            ready: AtomicBool::new(false),
            cancelled: AtomicBool::new(false),
            publish_order,
            read_order,
        }
    }

    /// Publishes `value` unless cancelled; returns whether it published.
    fn publish(&self, value: u32) -> bool {
        if self.cancelled.load(Ordering::Acquire) {
            return false;
        }
        self.value.store(value, Ordering::Relaxed);
        self.ready.store(true, self.publish_order);
        true
    }

    /// Returns the payload if the producer has raised `ready`.
    fn try_take(&self) -> Option<u32> {
        self.ready
            .load(self.read_order)
            .then(|| self.value.load(Ordering::Relaxed))
    }

    /// Asks the producer to stop before it publishes.
    fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    /// Returns whether the payload was published.
    fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Acquire)
    }
}

/// Explores every bounded interleaving of one producer and one consumer.
///
/// Panics (inside the model) when the protocol invariant is violated.
fn check_protocol(publish_order: Ordering, read_order: Ordering) {
    // A small preemption bound keeps the state space tractable and the run
    // deterministic; Loom explores all schedules within the bound.
    let mut builder = Builder::new();
    builder.preemption_bound = Some(3);
    builder.check(move || {
        let slot = Arc::new(Slot::new(publish_order, read_order));
        let producer = {
            let slot = Arc::clone(&slot);
            thread::spawn(move || slot.publish(42))
        };

        if let Some(value) = slot.try_take() {
            assert_eq!(
                value, 42,
                "a reader that saw `ready` must see the payload"
            );
        }
        slot.cancel();

        let published = producer.join().expect("the producer should not panic");
        assert_eq!(published, slot.is_ready(), "ready is set iff published");
    });
}

/// Checks the correct protocol: `Release` on publish pairs with `Acquire` on
/// read.
fn run_model() {
    check_protocol(Ordering::Release, Ordering::Acquire);
    println!(
        "Loom explored the publication and cancellation model without a violation."
    );
}

/// Runs the model.
fn main() {
    run_model();
}
// ANCHOR_END: example

/// Runs the model for the workspace example runner.
pub fn run() {
    run_model();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the release/acquire protocol holds in every explored
    /// interleaving.
    #[test]
    fn loom_release_acquire_protocol_holds() {
        check_protocol(Ordering::Release, Ordering::Acquire);
    }

    /// Verifies that Loom finds the stale payload when `ready` is raised with
    /// `Relaxed`.
    #[test]
    #[should_panic(expected = "must see the payload")]
    fn loom_relaxed_publication_is_rejected() {
        check_protocol(Ordering::Relaxed, Ordering::Relaxed);
    }

    #[test]
    fn test_loom_model_example_entry_point() {
        main();
    }
}
