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
mod mlua2;

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
    /// Runs the `mlua2` example.
    #[cfg(feature = "lua")]
    #[command(name = "mlua2")]
    Mlua2,
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
                Commands::Mlua2 => {
                    mlua2::run()?;
                }
            }
        }
    }
    Ok(())
}

/// Runs the example selected on the command line.
#[cfg(not(feature = "lua"))]
fn main() {}
