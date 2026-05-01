use clap::Parser;
use clap::Subcommand;

mod cssparser;
mod html5ever;
mod tl;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "cssparser")]
    Cssparser,
    #[command(name = "html5ever")]
    Html5ever,
    #[command(name = "tl")]
    Tl,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Cssparser => {
                let _ = cssparser::run();
            }
            Commands::Html5ever => {
                let _ = html5ever::run();
            }
            Commands::Tl => {
                let _ = tl::run();
            }
        }
    }
}
