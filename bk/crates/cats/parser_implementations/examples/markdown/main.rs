use clap::Parser;
use clap::Subcommand;

mod comrak;
mod markdown;
mod pulldown_cmark;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "comrak")]
    Comrak,
    #[command(name = "markdown")]
    Markdown,
    #[command(name = "pulldown_cmark")]
    PulldownCmark,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Comrak => {
                let _ = comrak::run();
            }
            Commands::Markdown => {
                let _ = markdown::run();
            }
            Commands::PulldownCmark => {
                let _ = pulldown_cmark::run();
            }
        }
    }
}
