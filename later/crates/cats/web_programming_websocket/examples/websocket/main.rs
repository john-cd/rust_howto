//! Runs the WebSocket examples.
//!
//! TODO Reviewed ??-??-2026.

/// An asynchronous WebSocket client using async-tungstenite's Tokio adapter.
mod async_tungstenite;
/// A Tokio WebSocket client and one-connection echo server.
mod tokio_tungstenite;
/// A blocking WebSocket client using tungstenite.
mod tungstenite;

/// Runs each of the WebSocket examples.
fn main() -> anyhow::Result<()> {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("async-tungstenite"));
    command = command.subcommand(clap::Command::new("tungstenite"));
    command = command.subcommand(clap::Command::new("tokio-tungstenite"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("async-tungstenite", _)) => {
            async_tungstenite::run()?;
            Ok(())
        }
        Some(("tungstenite", _)) => {
            tungstenite::run()?;
            Ok(())
        }
        Some(("tokio-tungstenite", _)) => {
            tokio_tungstenite::run()?;
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
