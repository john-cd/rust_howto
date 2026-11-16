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
    let mut command = clap::Command::new("examples");
    #[cfg(windows)]
    {
        command = command.subcommand(clap::Command::new("job-object"));
    }
    command = command.subcommand(clap::Command::new("kill-guard"));
    #[cfg(unix)]
    {
        command = command.subcommand(clap::Command::new("process-group"));
    }
    command = command.subcommand(clap::Command::new("timeout-kill"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        #[cfg(windows)]
        Some(("job-object", _)) => {
            job_object::run()?;
            Ok(())
        }
        Some(("kill-guard", _)) => {
            kill_guard::run()?;
            Ok(())
        }
        #[cfg(unix)]
        Some(("process-group", _)) => {
            process_group::run()?;
            Ok(())
        }
        Some(("timeout-kill", _)) => {
            timeout_kill::run()?;
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
