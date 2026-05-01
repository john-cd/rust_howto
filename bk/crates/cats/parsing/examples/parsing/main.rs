use clap::Parser;
use clap::Subcommand;

mod nom;
mod tree_sitter;
mod winnow;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "nom")]
    Nom,
    #[command(name = "tree_sitter")]
    TreeSitter,
    #[command(name = "winnow")]
    Winnow,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Nom => {
                let _ = nom::run();
            }
            Commands::TreeSitter => {
                let _ = tree_sitter::run();
            }
            Commands::Winnow => {
                let _ = winnow::run();
            }
        }
    }
}
