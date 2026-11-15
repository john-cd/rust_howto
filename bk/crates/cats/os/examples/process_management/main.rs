//! Runs the process management examples.
//!
//! TODO Reviewed ??-??-2026.

/// Make a Windows child die with its parent by using a Job Object.
#[cfg(windows)]
mod job_object;
/// Kill a child process automatically when its owner goes out of scope.
mod kill_guard;
/// Kill a child and all of its descendants with a Unix process group.
#[cfg(unix)]
mod process_group;
/// Run a command with a timeout and kill it if it takes too long.
mod timeout_kill;

/// Runs the examples that are available on this platform.
fn main() -> anyhow::Result<()> {
    #[cfg(unix)]
    process_group::run()?;
    #[cfg(windows)]
    job_object::run()?;
    timeout_kill::run()?;
    kill_guard::run()?;
    Ok(())
}
