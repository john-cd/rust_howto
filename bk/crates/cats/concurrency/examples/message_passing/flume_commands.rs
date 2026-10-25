#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Drains pending commands once per "tick" from a bounded `flume` channel.

use std::thread;
use std::time::Duration;

use flume::RecvTimeoutError;

/// A command sent to the worker.
#[derive(Debug, PartialEq)]
enum Command {
    /// Add a value to the running total.
    Add(i32),
    /// Stop the worker.
    Stop,
}

/// Processes commands tick by tick and returns the final total.
///
/// Each tick waits (up to a timeout) for the first command, then drains the
/// rest without blocking using `try_iter`.
fn worker(rx: &flume::Receiver<Command>) -> i32 {
    let mut total = 0;
    loop {
        // Wait for the first pending command; the timeout bounds the tick.
        let first = match rx.recv_timeout(Duration::from_secs(5)) {
            Ok(command) => command,
            Err(RecvTimeoutError::Timeout | RecvTimeoutError::Disconnected) => {
                return total;
            }
        };
        // Then handle everything else already queued, without blocking.
        for command in std::iter::once(first).chain(rx.try_iter()) {
            match command {
                Command::Add(n) => total += n,
                Command::Stop => return total,
            }
        }
    }
}

/// Sends commands from several threads (MPMC) and returns the total.
fn run_commands() -> i32 {
    let (tx, rx) = flume::bounded(8);
    let handles: Vec<_> = (1..=3)
        .map(|i| {
            let tx = tx.clone();
            thread::spawn(move || tx.send(Command::Add(i)).is_ok())
        })
        .collect();
    // Join the producers first, so `Stop` is guaranteed to arrive last.
    for handle in handles {
        if !handle.join().unwrap_or(false) {
            eprintln!("a producer failed to send");
        }
    }
    if tx.send(Command::Stop).is_err() {
        eprintln!("the worker has gone away");
    }
    worker(&rx)
}

/// Runs the example.
fn main() {
    println!("total: {}", run_commands());
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies all commands are applied.
    #[test]
    fn commands_are_drained() {
        assert_eq!(run_commands(), 6);
    }

    /// Verifies the worker stops when all senders are dropped.
    #[test]
    fn disconnect_ends_worker() {
        let (tx, rx) = flume::unbounded();
        tx.send(Command::Add(2)).expect("receiver is alive");
        drop(tx);
        assert_eq!(worker(&rx), 2);
    }
}
