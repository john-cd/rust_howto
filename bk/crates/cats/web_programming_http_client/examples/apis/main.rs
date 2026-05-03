use clap::Parser;
use clap::Subcommand;

mod paginated;
mod rate_limited;
mod rest_get;
mod rest_head;
mod rest_post;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "paginated")]
    Paginated,
    #[command(name = "rate_limited")]
    RateLimited,
    #[command(name = "rest_get")]
    RestGet,
    #[command(name = "rest_head")]
    RestHead,
    #[command(name = "rest_post")]
    RestPost,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Paginated => {
                if let Err(err) = paginated::run() {
                    eprintln!("{err}");
                }
            }
            Commands::RateLimited => {
                rate_limited::run();
            }
            Commands::RestGet => {
                rest_get::run();
            }
            Commands::RestHead => {
                rest_head::run();
            }
            Commands::RestPost => {
                rest_post::run();
            }
        }
    }
}
