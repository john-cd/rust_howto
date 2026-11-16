//! Runs the AES-GCM and ChaCha20-Poly1305 encryption examples.
//!
//! TODO Reviewed ??-??-2026.

/// Authenticated encryption and decryption with AES-256-GCM.
mod aes_gcm;
/// Authenticated encryption with ChaCha20-Poly1305, with and without associated
mod chacha20poly1305;

/// Runs both authenticated-encryption examples.
fn main() -> anyhow::Result<()> {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("aes-gcm"));
    command = command.subcommand(clap::Command::new("chacha20poly1305"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("aes-gcm", _)) => {
            aes_gcm::run()?;
            Ok(())
        }
        Some(("chacha20poly1305", _)) => {
            chacha20poly1305::run()?;
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
