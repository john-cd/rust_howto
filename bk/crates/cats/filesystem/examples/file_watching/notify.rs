#![allow(unused_imports)]
#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use the `notify` crate to watch for file
//! system events.
//!
//! It sets up a file system watcher that monitors the current directory and its
//! subdirectories for any changes, such as file creation, modification, or
//! deletion.

use std::path::Path;

use notify::EventHandler;
use notify::RecursiveMode;
use notify::Result;
use notify::Watcher;
use notify::event::Event;
use notify::event::EventKind;

/// Prints received events.
struct EventPrinter;

impl EventHandler for EventPrinter {
    /// This is the only method that needs to be implemented for the
    /// EventHandler trait. It is called when an event occurs in the watched
    /// directory.
    fn handle_event(&mut self, res_event: Result<Event>) {
        match res_event {
            Ok(event) => println!("event: {event:?}"),
            Err(e) => println!("watch error: {e:?}"),
        }
    }
}

/// Sets up and runs the file system watcher.
///
/// This function initializes a file system watcher using the `notify` crate.
/// The `EventPrinter` struct is used as the event handler, which in our case
/// simply prints any received events to the console.
fn main() -> Result<()> {
    // Automatically selects the best file system watcher implementation for the
    // current platform.
    let mut watcher = notify::recommended_watcher(EventPrinter)?;

    // Adds the current directory (".") to be watched.
    // `RecursiveMode::Recursive` ensures that all subdirectories are also
    // monitored for changes.
    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;
    println!("Watching current directory for changes...");

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
