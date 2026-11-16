//! Session login with `axum-login`, `tower-sessions` and Argon2.
//!
//! TODO Reviewed ??-??-2026.

/// Cookie-based login with `axum-login`, in-memory sessions and permissions.
mod axum_login;
/// Protect a state-changing form with a synchronizer CSRF token.
mod csrf;
/// Hash and verify passwords with Argon2id.
mod hashing;

/// Runs the password hashing, CSRF, and Axum session-login demonstrations.
///
/// # Errors
///
/// Returns an error if the password-hashing or Axum session-login example fails.
fn main() -> anyhow::Result<()> {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("hashing"));
    command = command.subcommand(clap::Command::new("csrf"));
    command = command.subcommand(clap::Command::new("axum-login"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("hashing", _)) => {
            hashing::run()?;
            Ok(())
        }
        Some(("csrf", _)) => {
            csrf::run();
            Ok(())
        }
        Some(("axum-login", _)) => {
            axum_login::run()?;
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
