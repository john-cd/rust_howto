//! Runs one of the `attributes` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod allow_dead_code;
mod attributes_deprecated;
mod attributes_derive;
mod attributes_early_development;
mod attributes_must_use;
mod attributes_production;
mod cfg_if;
mod conditional_compilation;

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
    /// Runs the `allow_dead_code` example.
    #[command(name = "allow_dead_code")]
    AllowDeadCode,
    /// Runs the `attributes_deprecated` example.
    #[command(name = "attributes_deprecated")]
    AttributesDeprecated,
    /// Runs the `attributes_derive` example.
    #[command(name = "attributes_derive")]
    AttributesDerive,
    /// Runs the `attributes_early_development` example.
    #[command(name = "attributes_early_development")]
    AttributesEarlyDevelopment,
    /// Runs the `attributes_must_use` example.
    #[command(name = "attributes_must_use")]
    AttributesMustUse,
    /// Runs the `attributes_production` example.
    #[command(name = "attributes_production")]
    AttributesProduction,
    /// Runs the `cfg_if` example.
    #[command(name = "cfg_if")]
    CfgIf,
    /// Runs the `conditional_compilation` example.
    #[command(name = "conditional_compilation")]
    ConditionalCompilation,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AllowDeadCode => {
                allow_dead_code::run();
            }
            Commands::AttributesDeprecated => {
                attributes_deprecated::run();
            }
            Commands::AttributesDerive => {
                attributes_derive::run();
            }
            Commands::AttributesEarlyDevelopment => {
                attributes_early_development::run();
            }
            Commands::AttributesMustUse => {
                attributes_must_use::run();
            }
            Commands::AttributesProduction => {
                attributes_production::run();
            }
            Commands::CfgIf => {
                cfg_if::run();
            }
            Commands::ConditionalCompilation => {
                conditional_compilation::run();
            }
        }
    }
}
