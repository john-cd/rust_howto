//! Runs one of the `iterators` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod consume_intoiterator;
mod implement_intoiterator;
mod iterator_adapters;
mod iterators;
mod return_iterator;
mod simple_iterators;

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
    /// Runs the `consume_intoiterator` example.
    #[command(name = "consume_intoiterator")]
    ConsumeIntoiterator,
    /// Runs the `implement_intoiterator` example.
    #[command(name = "implement_intoiterator")]
    ImplementIntoiterator,
    /// Runs the `iterator_adapters` example.
    #[command(name = "iterator_adapters")]
    IteratorAdapters,
    /// Runs the `iterators` example.
    #[command(name = "iterators")]
    Iterators,
    /// Runs the `return_iterator` example.
    #[command(name = "return_iterator")]
    ReturnIterator,
    /// Runs the `simple_iterators` example.
    #[command(name = "simple_iterators")]
    SimpleIterators,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::ConsumeIntoiterator => {
                consume_intoiterator::run();
            }
            Commands::ImplementIntoiterator => {
                implement_intoiterator::run();
            }
            Commands::IteratorAdapters => {
                iterator_adapters::run();
            }
            Commands::Iterators => {
                iterators::run();
            }
            Commands::ReturnIterator => {
                return_iterator::run();
            }
            Commands::SimpleIterators => {
                simple_iterators::run();
            }
        }
    }
}
