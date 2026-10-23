//! Runs one of the `lifetimes` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod lifetime_parameter_function;
mod lifetime_parameter_struct;
mod lifetime_parameters;
mod self_referential_struct;
mod self_referential_struct2;
mod static_lifetime;

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
    /// Runs the `lifetime_parameter_function` example.
    #[command(name = "lifetime_parameter_function")]
    LifetimeParameterFunction,
    /// Runs the `lifetime_parameter_struct` example.
    #[command(name = "lifetime_parameter_struct")]
    LifetimeParameterStruct,
    /// Runs the `lifetime_parameters` example.
    #[command(name = "lifetime_parameters")]
    LifetimeParameters,
    /// Runs the `self_referential_struct` example.
    #[command(name = "self_referential_struct")]
    SelfReferentialStruct,
    /// Runs the `self_referential_struct2` example.
    #[command(name = "self_referential_struct2")]
    SelfReferentialStruct2,
    /// Runs the `static_lifetime` example.
    #[command(name = "static_lifetime")]
    StaticLifetime,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::LifetimeParameterFunction => {
                lifetime_parameter_function::run();
            }
            Commands::LifetimeParameterStruct => {
                lifetime_parameter_struct::run();
            }
            Commands::LifetimeParameters => {
                lifetime_parameters::run();
            }
            Commands::SelfReferentialStruct => {
                self_referential_struct::run();
            }
            Commands::SelfReferentialStruct2 => {
                self_referential_struct2::run();
            }
            Commands::StaticLifetime => {
                static_lifetime::run();
            }
        }
    }
}
