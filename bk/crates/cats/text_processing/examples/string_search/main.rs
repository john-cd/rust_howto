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

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AhoCorasick => {
                let _ = aho_corasick::run();
            }
            Commands::FuzzyMatcher => {
                let _ = fuzzy_matcher::run();
            }
            Commands::Memchr => {
                let _ = memchr::run();
            }
            Commands::Strsim => {
                let _ = strsim::run();
            }
        }
    }
}
