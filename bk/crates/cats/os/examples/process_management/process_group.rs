// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Kill a child and all of its descendants with a Unix process group.

use std::os::unix::process::CommandExt as _;
use std::process::Child;
use std::process::Command;

use nix::sys::signal::Signal;
use nix::sys::signal::killpg;
use nix::unistd::Pid;

/// Starts `sh -c <script>` as the leader of a new process group.
fn spawn_in_new_group(script: &str) -> std::io::Result<Child> {
    // `process_group(0)` makes the child the leader of a new group whose id
    // equals the child's pid, so the signal below reaches its descendants too.
    Command::new("sh")
        .args(["-c", script])
        .process_group(0)
        .spawn()
}

/// Sends `SIGKILL` to every process in the child's group, then reaps the child.
fn kill_group(child: &mut Child) -> anyhow::Result<()> {
    let group = Pid::from_raw(i32::try_from(child.id())?);
    killpg(group, Signal::SIGKILL)?;
    // Always `wait` to reap the child; otherwise it stays a zombie.
    child.wait()?;
    Ok(())
}

/// Starts a shell that spawns a long-running grandchild, then kills the group.
pub fn run() -> anyhow::Result<()> {
    let mut child = spawn_in_new_group("sleep 60 & wait")?;
    kill_group(&mut child)?;
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use std::io::BufRead as _;
    use std::io::BufReader;
    use std::process::Stdio;
    use std::thread;
    use std::time::Duration;

    use nix::sys::signal::kill;

    use super::*;

    /// Returns true if the process exists and is not a zombie. A killed
    /// orphan stays a zombie until its new parent reaps it, which a minimal
    /// container init may not do.
    fn is_running(pid: Pid) -> bool {
        if kill(pid, None).is_err() {
            return false;
        }
        match std::fs::read_to_string(format!("/proc/{pid}/stat")) {
            // The state is the first field after the parenthesized name.
            Ok(stat) => !stat
                .rsplit(')')
                .next()
                .is_some_and(|rest| rest.trim_start().starts_with('Z')),
            Err(_) => true,
        }
    }

    /// Confirms that terminating a group also terminates its grandchild.
    #[test]
    fn grandchild_is_killed_with_the_group() -> anyhow::Result<()> {
        // The shell prints the pid of its background `sleep` and waits.
        let mut command = Command::new("sh");
        command
            .args(["-c", "sleep 60 & echo $!; wait"])
            .process_group(0)
            .stdout(Stdio::piped());
        let mut child = command.spawn()?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow::anyhow!("no stdout"))?;
        let mut line = String::new();
        BufReader::new(stdout).read_line(&mut line)?;
        let grandchild = Pid::from_raw(line.trim().parse()?);

        // The grandchild is alive.
        assert!(is_running(grandchild));

        kill_group(&mut child)?;

        // It is gone: poll briefly, since the kernel may need a moment.
        let mut gone = false;
        for _ in 0..50 {
            if !is_running(grandchild) {
                gone = true;
                break;
            }
            thread::sleep(Duration::from_millis(20));
        }
        assert!(gone);
        Ok(())
    }

    /// Verifies that a completed group command returns normally.
    #[test]
    fn run_completes() -> anyhow::Result<()> {
        run()
    }
}
