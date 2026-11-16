//! Describe an HTTP API and its data types.
//!
//!
//! OpenAPI is a standard, machine-readable description of an HTTP API: its
//! paths, parameters and data types. Tools use it to generate documentation
//! pages, client libraries and request validators, which keeps the server and
//! its callers in agreement.
//!
//! TODO Reviewed ??-??-2026.

/// Derive a JSON Schema with [`schemars`] and validate data against it with
mod schemars;
/// Generate an OpenAPI document from code with [`utoipa`].
mod utoipa;

/// Prints the generated OpenAPI document, propagating serialization errors.
fn main() -> anyhow::Result<()> {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("utoipa"));
    command = command.subcommand(clap::Command::new("schemars"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("utoipa", _)) => {
            utoipa::run()?;
            Ok(())
        }
        Some(("schemars", _)) => {
            schemars::run()?;
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
