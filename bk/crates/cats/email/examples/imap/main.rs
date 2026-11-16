//! Read mail over IMAP and filter disposable addresses.
//!
//! TODO Reviewed ??-??-2026.

/// Fetch unread messages over IMAP and parse them with [`mail_parser`].
mod imap_fetch;
/// Reject syntactically invalid and disposable addresses with [`mailchecker`].
mod mailchecker;

/// Runs the example.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("mailchecker"));
    command = command.subcommand(clap::Command::new("imap-fetch"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("mailchecker", _)) => {
            mailchecker::run();
            Ok(())
        }
        Some(("imap-fetch", _)) => {
            imap_fetch::run().await?;
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
