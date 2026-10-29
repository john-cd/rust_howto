// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! notify = "8.2.0"
//! ```

use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use notify::RecursiveMode;
use notify::Watcher as _;

/// Watches a temporary directory and waits briefly for a file-system event.
fn main() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    println!("Watching {} for changes...", temp_dir.path().display());
    let event = watch_for_write(temp_dir.path())?;
    println!("Event: {event:?}");
    Ok(())
}

/// Watches `dir`, creates a file in it, and returns the first event.
fn watch_for_write(dir: &Path) -> anyhow::Result<notify::Event> {
    let (sender, receiver) = mpsc::channel();
    let mut watcher = notify::recommended_watcher(sender)?;
    watcher.watch(dir, RecursiveMode::Recursive)?;

    // Create a file after starting the watcher so it can report the event.
    let path = dir.join("example.txt");
    std::fs::write(path, "trigger an event")?;

    match receiver.recv_timeout(Duration::from_secs(5)) {
        Ok(event) => Ok(event?),
        Err(mpsc::RecvTimeoutError::Timeout) => Err(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "No file-system event received within five seconds.",
        )
        .into()),
        Err(error) => Err(error.into()),
    }
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

    /// Creating a file produces an event naming that file.
    #[test]
    fn reports_the_created_file() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let event = watch_for_write(dir.path())?;
        assert!(
            event.paths.iter().any(|p| p.ends_with("example.txt")),
            "{event:?}"
        );
        Ok(())
    }

    /// Watching a missing directory fails.
    #[test]
    fn missing_directory_is_an_error() {
        assert!(watch_for_write(Path::new("/definitely/not/here")).is_err());
    }
}
