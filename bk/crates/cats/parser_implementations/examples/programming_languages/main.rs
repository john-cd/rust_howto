//! Runs the SQL parser example when the `sqlparser` feature is enabled.
//!
//! TODO Reviewed ??-??-2026.

/// Contains the sqlparser functionality used by this parent module.
#[cfg(feature = "sqlparser")]
mod sqlparser;
/// Contains the syn functionality used by this parent module.
mod syn;

/// Runs the example if the `sqlparser` feature is enabled.
fn main() -> anyhow::Result<()> {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("syn"));
    #[cfg(feature = "sqlparser")]
    {
        command = command.subcommand(clap::Command::new("sqlparser"));
    }
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("syn", _)) => {
            syn::run()?;
            Ok(())
        }
        #[cfg(feature = "sqlparser")]
        Some(("sqlparser", _)) => {
            sqlparser::run()?;
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
