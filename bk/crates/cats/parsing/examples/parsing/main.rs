// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod nom;
mod tree_sitter;
mod winnow;

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
    /// Runs the `nom` example.
    #[command(name = "nom")]
    Nom,
    /// Runs the `tree_sitter` example.
    #[command(name = "tree_sitter")]
    TreeSitter,
    /// Runs the `winnow` example.
    #[command(name = "winnow")]
    Winnow,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Nom => {
                nom::run();
            }
            Commands::TreeSitter => {
                tree_sitter::run()?;
            }
            Commands::Winnow => {
                winnow::run()?;
            }
        }
    }

    Ok(())
}
