// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Run a command with a timeout and kill it if it takes too long.

use std::process::Child;
use std::process::Command;
use std::process::ExitStatus;
use std::time::Duration;

use wait_timeout::ChildExt as _;

/// Waits up to `limit` for `child`; returns `None` after killing it on timeout.
fn wait_or_kill(
    child: &mut Child,
    limit: Duration,
) -> std::io::Result<Option<ExitStatus>> {
    if let Some(status) = child.wait_timeout(limit)? {
        return Ok(Some(status));
    }
    // The child is still running: kill it, then reap it to avoid a zombie.
    child.kill()?;
    child.wait()?;
    Ok(None)
}

/// Returns a command that sleeps for roughly `seconds` seconds.
fn sleep_command(seconds: u32) -> Command {
    if cfg!(windows) {
        let mut command = Command::new("powershell.exe");
        command.args(["-NoLogo", "-NoProfile", "-NonInteractive", "-Command"]);
        if seconds == 0 {
            command.arg("exit 0");
        } else {
            command.arg(format!("Start-Sleep -Seconds {seconds}"));
        }
        command.stdout(std::process::Stdio::null());
        command
    } else {
        let mut command = Command::new("sleep");
        command.arg(seconds.to_string());
        command
    }
}

/// Kills a slow command after a timeout.
pub fn run() -> anyhow::Result<()> {
    let mut child = sleep_command(30).spawn()?;
    let outcome = wait_or_kill(&mut child, Duration::from_millis(200))?;
    assert!(outcome.is_none(), "the command should have timed out");
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Terminates and reaps a command that exceeds its deadline.
    #[test]
    fn slow_command_times_out() -> anyhow::Result<()> {
        run()
    }

    /// Preserves the output of a command that finishes before its deadline.
    #[test]
    fn fast_command_completes() -> anyhow::Result<()> {
        let mut child = sleep_command(0).spawn()?;
        let outcome = wait_or_kill(&mut child, Duration::from_secs(10))?;
        assert!(outcome.is_some_and(|status| status.success()));
        Ok(())
    }
}
