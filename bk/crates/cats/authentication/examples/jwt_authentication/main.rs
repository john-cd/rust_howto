//! Signs and verifies a JWT and demonstrates an Axum bearer-token extractor.
//!
//! TODO Reviewed ??-??-2026.

/// Encode and verify HS256 JSON Web Tokens for a first-party API.
mod jwt;
/// Extract a JWT from the HTTP `Authorization` header and check a role in Axum.
mod jwt_axum_extractor;

/// Demonstrates creating, signing, and verifying a JSON Web Token (JWT) as well as configuring an
/// Axum router protected by bearer-token authentication.
mod jwt_service;

/// Signs and verifies an in-memory token using the `JWT_SECRET` environment
///
/// variable if set, otherwise falling back to the test secret for demonstration.
///
/// # Errors
///
/// Returns an error if token creation or verification fails.
fn main() -> anyhow::Result<()> {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("jwt"));
    command = command.subcommand(clap::Command::new("jwt-axum-extractor"));
    command = command.subcommand(clap::Command::new("jwt-service"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("jwt", _)) => {
            jwt::run()?;
            Ok(())
        }
        Some(("jwt-axum-extractor", _)) => {
            let _router = jwt_axum_extractor::run()?;
            Ok(())
        }
        Some(("jwt-service", _)) => {
            jwt_service::run()?;
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
