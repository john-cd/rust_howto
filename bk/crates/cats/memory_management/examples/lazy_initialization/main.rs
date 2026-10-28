//! Runs one of the `lazy_initialization` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod lazy_cell;
mod lazy_constant;
mod lazy_lock;
mod lazy_static;
mod once_cell;
mod once_cell2;
mod once_lock;

/// Command-line arguments.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[derive(Subcommand)]
enum Commands {
    /// Runs the `lazy_cell` example.
    #[command(name = "lazy_cell")]
    LazyCell,
    /// Runs the `lazy_lock` example.
    #[command(name = "lazy_lock")]
    LazyLock,
    /// Runs the `lazy_constant` example.
    #[command(name = "lazy_constant")]
    LazyConstant,
    /// Runs the `lazy_static` example.
    #[command(name = "lazy_static")]
    LazyStatic,
    /// Runs the `once_cell` example.
    #[command(name = "once_cell")]
    OnceCell,
    /// Runs the `once_cell2` example.
    #[command(name = "once_cell2")]
    OnceCell2,
    /// Runs the `once_lock` example.
    #[command(name = "once_lock")]
    OnceLock,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::LazyCell => lazy_cell::run(),
            Commands::LazyLock => lazy_lock::run(),
            Commands::LazyConstant => {
                lazy_constant::run();
            }
            Commands::LazyStatic => {
                lazy_static::run()?;
            }
            Commands::OnceCell => {
                once_cell::run();
            }
            Commands::OnceCell2 => {
                once_cell2::run();
            }
            Commands::OnceLock => once_lock::run(),
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_cli_dispatcher() {
        let _ = Cli::try_parse_from(["example"]);
    }
}
