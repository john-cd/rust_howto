//! Runs the graceful shutdown examples.
//!
//! TODO Reviewed ??-??-2026.

/// Shut a worker thread down cleanly when the user presses Ctrl-C.
mod ctrlc_example;
/// Shut an async service down on Ctrl-C or SIGTERM with Tokio.
mod tokio_signal;

/// Runs both examples.
fn main() -> anyhow::Result<()> {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("ctrlc"));
    command = command.subcommand(clap::Command::new("tokio-signal"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("ctrlc", _)) => {
            ctrlc_example::run()?;
            Ok(())
        }
        Some(("tokio-signal", _)) => {
            tokio_signal::run()?;
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
