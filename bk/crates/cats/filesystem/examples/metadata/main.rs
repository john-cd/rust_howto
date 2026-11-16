//! Runs the file metadata example.
//!
//! TODO Reviewed ??-??-2026.

/// Detect a file's type from its first bytes.
mod infer;
/// Reading the metadata of files and directories.
mod metadata;

/// Runs the metadata example.
fn main() -> anyhow::Result<()> {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("metadata"));
    command = command.subcommand(clap::Command::new("infer"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("metadata", _)) => {
            metadata::run()?;
            Ok(())
        }
        Some(("infer", _)) => {
            infer::run()?;
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
