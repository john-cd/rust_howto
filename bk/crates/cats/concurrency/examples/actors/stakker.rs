#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! An actor with the `stakker` crate: a counter that processes messages.

use std::time::Instant;

use stakker::*;

/// An actor that holds a running total.
struct Counter {
    /// The running total.
    count: u32,
}

impl Counter {
    /// Creates a counter actor with an initial value of zero.
    fn init(_cx: CX![]) -> Option<Self> {
        Some(Self { count: 0 })
    }

    /// Adds an amount to the counter.
    fn increment(&mut self, _cx: CX![], amount: u32) {
        self.count += amount;
    }

    /// Prints the current counter value.
    fn report(&self, _cx: CX![]) {
        println!("Current value: {}", self.count);
    }
}

/// Spawns a [`Counter`], sends it messages and runs the system.
fn main() {
    let mut system = Stakker::new(Instant::now());
    let stakker = &mut system;
    let counter = actor!(stakker, Counter::init(), ret_nop!());

    call!([counter], increment(5));
    call!([counter], increment(10));
    call!([counter], report());

    stakker.run(Instant::now(), false);
}
// ANCHOR_END: example

/// Runs the Stakker actor example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the stakker example through its entry point.
    #[test]
    fn test_stakker_example_entry_point() {
        main();
    }
}
