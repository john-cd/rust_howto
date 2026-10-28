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
fn main() -> LuaResult<()> {
    #[cfg(feature = "lua")]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                #[cfg(feature = "lua")]
                Commands::Mlua => {
                    mlua::run()?;
                }
                #[cfg(feature = "lua")]
                Commands::MluaScripting => {
                    mlua_scripting::run()?;
                }
                Commands::MluaUserdata => {
                    mlua_userdata::run()?;
                }
            }
        }
    }
    Ok(())
}

/// Runs the example selected on the command line.
#[cfg(not(feature = "lua"))]
fn main() {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_cli_dispatcher() {
        let _ = Cli::try_parse_from(["example"]);
    }
}
