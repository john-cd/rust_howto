//! Configuration management examples.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod config_hierarchical;
mod config_singleton;
mod config_testing;
mod confy;

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
    /// Runs the `config_hierarchical` example.
    #[command(name = "config_hierarchical")]
    ConfigHierarchical,
    /// Runs the `config_singleton` example.
    #[command(name = "config_singleton")]
    ConfigSingleton,
    /// Runs the `config_testing` example.
    #[command(name = "config_testing")]
    ConfigTesting,
    /// Runs the `confy` example.
    #[command(name = "confy")]
    Confy,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::ConfigHierarchical => {
                config_hierarchical::run()?;
            }
            Commands::ConfigSingleton => {
                config_singleton::run()?;
            }
            Commands::ConfigTesting => {
                config_testing::run()?;
            }
            Commands::Confy => {
                confy::run()?;
            }
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
