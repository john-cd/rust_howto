use clap::Parser;
use clap::Subcommand;

mod download;
mod partial;
mod post_file;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "download")]
    Download,
    #[command(name = "partial")]
    Partial,
    #[command(name = "post_file")]
    PostFile,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Download => {
                let _ = download::run();
            }
            Commands::Partial => {
                let _ = partial::run();
            }
            Commands::PostFile => {
                let _ = post_file::run();
            }
        }
    }
}
