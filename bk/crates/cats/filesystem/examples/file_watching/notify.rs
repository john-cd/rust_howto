#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! notify = "8.2.0"
//! ```

use std::sync::mpsc;
use std::time::Duration;

use notify::RecursiveMode;
use notify::Watcher as _;

/// Watches a temporary directory and waits briefly for a file-system event.
fn main() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let (sender, receiver) = mpsc::channel();
    let mut watcher = notify::recommended_watcher(sender)?;
    watcher.watch(temp_dir.path(), RecursiveMode::Recursive)?;

    println!("Watching {} for changes...", temp_dir.path().display());

    // Create a file after starting the watcher so it can report the event.
    let path = temp_dir.path().join("example.txt");
    std::fs::write(path, "trigger an event")?;

    match receiver.recv_timeout(Duration::from_secs(5)) {
        Ok(event) => println!("Event: {event:?}"),
        Err(mpsc::RecvTimeoutError::Timeout) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "No file-system event received within five seconds.",
            )
            .into());
        }
        Err(error) => return Err(error.into()),
    }

    Ok(())
}

// Look for additional examples in <https://github.com/notify-rs/notify/tree/main/examples>.
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the notify example through its entry point.
    #[test]
    fn test_notify_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
