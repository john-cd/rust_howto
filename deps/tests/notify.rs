#![allow(unused_imports)]
// ANCHOR: example

use std::path::Path;

use notify::event::Event;
use notify::EventHandler;
use notify::RecursiveMode;
use notify::Result;
use notify::Watcher;

/// Prints received events
struct EventPrinter;

impl EventHandler for EventPrinter {
    fn handle_event(&mut self, res_event: Result<Event>) {
        match res_event {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn main() -> Result<()> {
    // Automatically select the best implementation for your platform.
    let mut watcher = notify::recommended_watcher(EventPrinter)?;

    // Add a path to be watched. All files and directories at that path
    // and below will be monitored for changes.
    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

    Ok(())
}

// ANCHOR_END: example
#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
