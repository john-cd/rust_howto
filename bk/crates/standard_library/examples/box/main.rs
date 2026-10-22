// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod box_basics;
mod box_deref;
mod box_dst;
mod box_recursive;
mod box_trait_objects;

/// Command-line arguments.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
enum Commands {
    /// Runs the `box_basics` example.
    #[command(name = "box_basics")]
    BoxBasics,
    /// Runs the `box_deref` example.
    #[command(name = "box_deref")]
    BoxDeref,
    /// Runs the `box_dst` example.
    #[command(name = "box_dst")]
    BoxDst,
    /// Runs the `box_recursive` example.
    #[command(name = "box_recursive")]
    BoxRecursive,
    /// Runs the `box_trait_objects` example.
    #[command(name = "box_trait_objects")]
    BoxTraitObjects,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::BoxBasics => {
                box_basics::run();
            }
            Commands::BoxDeref => {
                box_deref::run();
            }
            Commands::BoxDst => {
                box_dst::run();
            }
            Commands::BoxRecursive => {
                box_recursive::run();
            }
            Commands::BoxTraitObjects => {
                box_trait_objects::run();
            }
        }
    }
}
