use clap::Parser;
use clap::Subcommand;

mod consume_intoiterator;
mod implement_intoiterator;
mod iterator_adapters;
mod iterators;
mod return_iterator;
mod simple_iterators;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "consume_intoiterator")]
    ConsumeIntoiterator,
    #[command(name = "implement_intoiterator")]
    ImplementIntoiterator,
    #[command(name = "iterator_adapters")]
    IteratorAdapters,
    #[command(name = "iterators")]
    Iterators,
    #[command(name = "return_iterator")]
    ReturnIterator,
    #[command(name = "simple_iterators")]
    SimpleIterators,
}

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
