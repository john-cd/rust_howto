// TODO Reviewed ??-??-2026.

#![expect(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! Kill a child process automatically when its owner goes out of scope.

use std::process::Child;
use std::process::Command;

/// Owns a child process and kills it on drop unless it was released.
struct KillOnDrop(Option<Child>);

impl KillOnDrop {
    /// Starts guarding `child`.
    fn new(child: Child) -> Self {
        Self(Some(child))
    }

    /// Returns the child without killing it.
    fn release(mut self) -> Option<Child> {
        self.0.take()
    }

    /// Returns the guarded child's process id, if it was not released.
    fn id(&self) -> Option<u32> {
        self.0.as_ref().map(Child::id)
    }
}

impl Drop for KillOnDrop {
    /// Kills and reaps the child, ignoring errors (for example if it already
    /// exited): `drop` cannot return them.
    fn drop(&mut self) {
        if let Some(child) = &mut self.0 {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

/// Returns a command that sleeps for roughly `seconds` seconds.
fn sleep_command(seconds: u32) -> Command {
    if cfg!(windows) {
        let mut command = Command::new("ping");
        command.args(["-n", &(seconds + 1).to_string(), "127.0.0.1"]);
        command.stdout(std::process::Stdio::null());
        command
    } else {
        let mut command = Command::new("sleep");
        command.arg(seconds.to_string());
        command
    }
}

/// Fails after starting a child; the guard cleans up on the early return.
fn fallible_work() -> anyhow::Result<()> {
    let guard = KillOnDrop::new(sleep_command(30).spawn()?);
    anyhow::bail!(
        "something went wrong after starting process {:?}",
        guard.id()
    );
}

/// Runs the failing function; the child does not outlive it.
pub fn run() -> anyhow::Result<()> {
    assert!(fallible_work().is_err());
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Keeps a child alive after the guard explicitly releases ownership.
    #[test]
    fn released_child_keeps_running() -> anyhow::Result<()> {
        let guard = KillOnDrop::new(sleep_command(30).spawn()?);
        let mut child = guard
            .release()
            .ok_or_else(|| anyhow::anyhow!("child was already released"))?;
        assert!(child.try_wait()?.is_none());
        child.kill()?;
        assert!(!child.wait()?.success());
        Ok(())
    }

    /// Verifies that a completed command is reaped normally.
    #[test]
    fn run_completes() -> anyhow::Result<()> {
        run()
    }
}
