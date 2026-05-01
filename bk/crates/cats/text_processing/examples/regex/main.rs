use clap::Parser;
use clap::Subcommand;

mod email;
mod fancy_regex;
mod filter_log;
mod hashtags;
mod phone;
mod regex;
mod replace;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "email")]
    Email,
    #[command(name = "fancy_regex")]
    FancyRegex,
    #[command(name = "filter_log")]
    FilterLog,
    #[command(name = "hashtags")]
    Hashtags,
    #[command(name = "phone")]
    Phone,
    #[command(name = "regex")]
    Regex,
    #[command(name = "replace")]
    Replace,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Email => {
                email::run();
            }
            Commands::FancyRegex => {
                fancy_regex::run();
            }
            Commands::FilterLog => {
                filter_log::run();
            }
            Commands::Hashtags => {
                hashtags::run();
            }
            Commands::Phone => {
                phone::run();
            }
            Commands::Regex => {
                regex::run();
            }
            Commands::Replace => {
                replace::run();
            }
        }
    }
}
