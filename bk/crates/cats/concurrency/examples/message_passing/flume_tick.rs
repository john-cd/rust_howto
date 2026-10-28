// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A bounded `flume` channel for commands sent to a loop that handles them
//! once per "tick" (a UI frame, a game loop iteration, a control cycle).

use std::thread;
use std::time::Duration;

use anyhow::Result;
use flume::RecvTimeoutError;
use flume::bounded;

/// A command sent to the loop.
#[derive(Debug, PartialEq)]
enum Command {
    /// Adds a value to the running total.
    Add(i32),
    /// Stops the loop.
    Quit,
}

/// Handles all pending commands without blocking.
///
/// Returns `true` if the loop should stop.
fn drain_commands(rx: &flume::Receiver<Command>, total: &mut i32) -> bool {
    // `try_iter` yields only the messages already in the channel.
    for command in rx.try_iter() {
        match command {
            Command::Add(n) => *total += n,
            Command::Quit => return true,
        }
    }
    false
}

/// Runs a controller thread and a tick loop connected by a bounded channel.
///
/// # Errors
///
/// Returns an error if the controller panics or the command receiver closes.
fn main() -> Result<()> {
    // A bounded MPMC channel: `send` blocks when it is full.
    let (tx, rx) = bounded::<Command>(4);

    let controller = thread::spawn(move || -> Result<()> {
        for n in 1..=10 {
            tx.send(Command::Add(n))?;
        }
        tx.send(Command::Quit)?;
        Ok(())
    });

    let mut total = 0;
    loop {
        // Once per tick, handle everything that is pending.
        if drain_commands(&rx, &mut total) {
            break;
        }
        // Then wait for more work, up to the tick length.
        match rx.recv_timeout(Duration::from_millis(1)) {
            Ok(Command::Add(n)) => total += n,
            Ok(Command::Quit) | Err(RecvTimeoutError::Disconnected) => break,
            Err(RecvTimeoutError::Timeout) => {} // No work this tick.
        }
    }

    controller
        .join()
        .map_err(|_| anyhow::anyhow!("command controller thread panicked"))??;

    assert_eq!(total, 55);
    println!("Total: {total}");
    Ok(())
}

/// An empty channel times out instead of blocking forever.
fn timeout_on_empty() {
    let (_tx, rx) = bounded::<Command>(1);
    assert_eq!(
        rx.recv_timeout(Duration::ZERO),
        Err(RecvTimeoutError::Timeout)
    );
}
// ANCHOR_END: example

/// Runs the command loop and reports controller failures.
///
/// # Errors
///
/// Returns an error if the controller panics or cannot send its commands.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the flume tick example through its entry point.
    #[test]
    fn test_flume_tick_example_entry_point() -> Result<()> {
        main()
    }

    /// Reports a timeout immediately when an open channel has no command.
    #[test]
    fn test_timeout_on_empty() {
        timeout_on_empty();
    }
}
