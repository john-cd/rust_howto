//! Runs one of the `lua` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(feature = "lua")]
use ::mlua::prelude::LuaResult;
#[cfg(feature = "lua")]
use clap::Parser;
#[cfg(feature = "lua")]
use clap::Subcommand;

#[cfg(feature = "lua")]
mod mlua;
#[cfg(feature = "lua")]
mod mlua_scripting;
#[cfg(feature = "lua")]
mod mlua_userdata;

/// Command-line arguments.
#[cfg(feature = "lua")]
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[cfg(feature = "lua")]
#[derive(Subcommand)]
enum Commands {
    /// Runs the `mlua` example.
    #[cfg(feature = "lua")]
    #[command(name = "mlua")]
    Mlua,
    /// Runs the embedded scripting example.
    #[cfg(feature = "lua")]
    #[command(name = "mlua_scripting")]
    MluaScripting,
    /// Runs the userdata example.
    #[cfg(feature = "lua")]
    #[command(name = "mlua_userdata")]
    MluaUserdata,
}

/// Runs the example selected on the command line.
#[cfg(feature = "lua")]
fn dispatch(cli: Cli) -> LuaResult<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::Mlua => mlua::run()?,
            Commands::MluaScripting => mlua_scripting::run()?,
            Commands::MluaUserdata => mlua_userdata::run()?,
        }
    }
    Ok(())
}

/// Parses process arguments and runs the selected Lua example.
#[cfg(feature = "lua")]
fn main() -> LuaResult<()> {
    dispatch(Cli::parse())
}

#[cfg(all(test, feature = "lua"))]
mod tests {
    use clap::CommandFactory;
    use clap::Parser;

    use super::*;

    /// Checks that every Lua example is reachable from the CLI.
    #[test]
    fn cli_lists_all_examples() {
        Cli::command().debug_assert();
        for name in ["mlua", "mlua_scripting", "mlua_userdata"] {
            let cli = Cli::try_parse_from(["example", name]).unwrap_or_else(
                |error| panic!("`{name}` should parse: {error}"),
            );
            assert!(cli.command.is_some());
        }
    }

    /// Runs the no-subcommand path without reading the test harness arguments.
    #[test]
    fn dispatch_without_a_subcommand_succeeds() -> LuaResult<()> {
        dispatch(Cli { command: None })
    }
}

/// Runs the example selected on the command line.
#[cfg(not(feature = "lua"))]
fn main() {}
