// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod aho_corasick;
mod fuzzy_matcher;
mod memchr;
mod strsim;

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
    /// Runs the `aho_corasick` example.
    #[command(name = "aho_corasick")]
    AhoCorasick,
    /// Runs the `fuzzy_matcher` example.
    #[command(name = "fuzzy_matcher")]
    FuzzyMatcher,
    /// Runs the `memchr` example.
    #[command(name = "memchr")]
    Memchr,
    /// Runs the `strsim` example.
    #[command(name = "strsim")]
    Strsim,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AhoCorasick => {
                aho_corasick::run()?;
            }
            Commands::FuzzyMatcher => {
                fuzzy_matcher::run();
            }
            Commands::Memchr => {
                memchr::run();
            }
            Commands::Strsim => {
                strsim::run();
            }
        }
    }
    Ok(())
}
