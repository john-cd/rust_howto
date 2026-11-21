// TODO Reviewed ??-??-2026.
//! Watches an editor-style save that replaces an existing file.

// ANCHOR: example
use std::io::Write;
use std::sync::mpsc;
use std::time::Duration;
use std::time::Instant;

use anyhow::Context;
use notify_debouncer_full::DebounceEventResult;
use notify_debouncer_full::new_debouncer;
use notify_debouncer_full::notify::RecursiveMode;

/// Writes a burst to a temporary file and checks that the replacement is reported.
///
/// # Errors
///
/// Returns an error if the file operations, watcher, or bounded wait fails.
fn main() -> anyhow::Result<()> {
    let directory = tempfile::tempdir()?;
    let path = directory.path().canonicalize()?;
    let target = path.join("notes.txt");
    std::fs::write(&target, b"old")?;
    let (sender, receiver) = mpsc::channel();
    let mut debouncer = new_debouncer(
        Duration::from_millis(150),
        None,
        move |batch: DebounceEventResult| {
            let _ = sender.send(batch);
        },
    )?;
    debouncer.watch(&path, RecursiveMode::NonRecursive)?;

    let mut staging = tempfile::NamedTempFile::new_in(&path)?;
    staging.write_all(b"new ")?;
    staging.write_all(b"contents")?;
    staging.persist(&target)?;
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let batch = receiver
            .recv_timeout(deadline.saturating_duration_since(Instant::now()))
            .context("waiting for the destination path")?;
        let events = batch.map_err(|errors| anyhow::anyhow!("watch errors: {errors:?}"))?;
        if events.iter().any(|entry| entry.paths.contains(&target)) {
            break;
        }
    }
    assert_eq!(std::fs::read(&target)?, b"new contents");
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Checks that the watched directory reports the replacement destination.
    #[test]
    fn sees_replaced_file() -> anyhow::Result<()> {
        super::main()
    }
}
