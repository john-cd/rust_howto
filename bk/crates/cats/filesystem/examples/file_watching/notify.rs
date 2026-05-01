#![allow(unused_imports)]
#![allow(dead_code)]
// ANCHOR: example
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! notify = "8.1.0" # Or latest.
//! ```
use std::path::Path;

use notify::EventHandler;
use notify::RecursiveMode;
use notify::Result;
use notify::Watcher;
use notify::event::Event;
use notify::event::EventKind;

struct EventPrinter;

impl EventHandler for EventPrinter {
    /// Prints received file watching events.
    /// This method is called when an event occurs in the watched
    /// directory.
    fn handle_event(&mut self, res_event: Result<Event>) {
        match res_event {
            Ok(event) => println!("Event: {event:?}"),
            Err(e) => println!("Watch error: {e:?}"),
        }
    }
}

fn main() -> Result<()> {
    // Initialize a file system watcher using the `notify` crate.
    // `recommended_watcher` automatically selects the best file system watcher
    // implementation for the current platform. The `EventHandler` passed to
    // this constructor can be a closure, a `std::sync::mpsc::Sender`, a
    // `crossbeam_channel::Sender`, or another type the trait is implemented
    // for. Here, a custom `EventPrinter` struct is used, which in our case
    // simply prints any received events to the console.
    let mut watcher = notify::recommended_watcher(EventPrinter)?;

    println!("Watching current directory for changes...");

    // Add a directory to be watched.
    // `RecursiveMode::Recursive` ensures that all subdirectories are also
    // monitored for changes.
    watcher.watch(Path::new("./temp"), RecursiveMode::Recursive)?;

    // Create folder(s) to generate example events:
    let path = "./temp/examples";
    std::fs::create_dir_all(path)?;

    Ok(())
}
// Look for additional examples in <https://github.com/notify-rs/notify/tree/main/examples>.
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
