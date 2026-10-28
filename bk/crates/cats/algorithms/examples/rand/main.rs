//! Runs one of the `rand` examples, chosen on the command line.
//!
//! Reviewed 10-27-2026.

use clap::Parser;
use clap::Subcommand;

mod fastrand;
mod rand1;
mod rand_alphanumeric;
mod rand_custom;
mod rand_distr;
mod rand_password;
mod rand_range;
mod rand_range_uniform;

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
    /// Runs the `fastrand` example.
    #[command(name = "fastrand")]
    Fastrand,
    /// Runs the `rand1` example.
    #[command(name = "rand")]
    Rand,
    /// Runs the `rand_alphanumeric` example.
    #[command(name = "rand_alphanumeric")]
    RandAlphanumeric,
    /// Runs the `rand_custom` example.
    #[command(name = "rand_custom")]
    RandCustom,
    /// Runs the `rand_distr` example.
    #[command(name = "rand_distr")]
    RandDistr,
    /// Runs the `rand_password` example.
    #[command(name = "rand_password")]
    RandPassword,
    /// Runs the `rand_range` example.
    #[command(name = "rand_range")]
    RandRange,
    /// Runs the `rand_range1` example.
    #[command(name = "rand_range_uniform")]
    RandRangeUniform,
}

/// Runs the example selected on the command line.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Fastrand => {
                fastrand::run();
            }
            Commands::Rand => {
                rand1::run();
            }
            Commands::RandAlphanumeric => {
                rand_alphanumeric::run();
            }
            Commands::RandCustom => {
                rand_custom::run();
            }
            Commands::RandDistr => {
                rand_distr::run()?;
            }
            Commands::RandPassword => {
                rand_password::run();
            }
            Commands::RandRange => {
                rand_range::run();
            }
            Commands::RandRangeUniform => {
                rand_range_uniform::run();
            }
        }
    }

    Ok(())
}
