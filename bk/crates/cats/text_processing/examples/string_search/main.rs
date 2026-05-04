use clap::Parser;
use clap::Subcommand;

mod aho_corasick;
mod fuzzy_matcher;
mod memchr;
mod strsim;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "aho_corasick")]
    AhoCorasick,
    #[command(name = "fuzzy_matcher")]
    FuzzyMatcher,
    #[command(name = "memchr")]
    Memchr,
    #[command(name = "strsim")]
    Strsim,
}

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
